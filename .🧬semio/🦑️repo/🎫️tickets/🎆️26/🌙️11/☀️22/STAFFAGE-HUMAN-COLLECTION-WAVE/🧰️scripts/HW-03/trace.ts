/** ✒️ Traces drawn outlines from the authored volume of the same scale in the default pose, the way the base humans were traced: the projected solids are rasterised, their outer contour simplified and carried back to the rest frame of the bone through the exact inverse of the engine's placement (an elevation shape by the bone's side affine; a front or plan shape with an elevation twin of the same id sits at the constant depth of the middle of that twin, so its carry is affine too).
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts — `placeProjected`, `depthProfile`, `carryPoint` */
import { type J, S, SCALES } from "./lib.ts";

const ev = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const sil = await import(`${S}/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts`);
const rig = await import(`${S}/🔨️modules/🦴️rig/🟦️.ts`);

type P = [number, number];
type View = "elevation" | "front" | "plan";

/** 🧭️ Outer contour (boundary walk on cell corners) of the largest filled component of a mask, in cell coordinates. */
function contour(mask: Uint8Array, w: number, h: number): P[] {
  const at = (i: number, j: number): number => (i >= 0 && j >= 0 && i < w && j < h ? mask[j * w + i]! : 0);
  let best: P[] = [];
  let bestArea = 0;
  const dirs: P[] = [[1, 0], [0, 1], [-1, 0], [0, -1]];
  const done = new Set<number>();
  for (let j = 0; j < h; j++)
    for (let i = 0; i < w; i++) {
      if (!at(i, j) || at(i, j - 1) || done.has(j * w + i)) continue;
      const ring: P[] = [];
      let x = i, y = j, dir = 0;
      for (let guard = 0; guard < 8 * (w + h) * 8; guard++) {
        ring.push([x, y]);
        if (dir === 0) done.add(y * w + x);
        let moved = false;
        for (const turn of [3, 0, 1, 2]) {
          const d = (dir + turn) % 4;
          const inside = d === 0 ? at(x, y) : d === 1 ? at(x - 1, y) : d === 2 ? at(x - 1, y - 1) : at(x, y - 1);
          const outside = d === 0 ? at(x, y - 1) : d === 1 ? at(x, y) : d === 2 ? at(x - 1, y) : at(x - 1, y - 1);
          if (inside && !outside) {
            x += dirs[d]![0];
            y += dirs[d]![1];
            dir = d;
            moved = true;
            break;
          }
        }
        if (!moved || (x === i && y === j)) break;
      }
      let area = 0;
      for (let k = 0; k < ring.length; k++) {
        const a = ring[k]!, b = ring[(k + 1) % ring.length]!;
        area += a[0] * b[1] - b[0] * a[1];
      }
      if (Math.abs(area) > bestArea) (bestArea = Math.abs(area)), (best = ring);
    }
  return best;
}

/** 〰️ Ramer-Douglas-Peucker on a closed ring. */
function simplify(ring: P[], tol: number): P[] {
  const rdp = (pts: P[]): P[] => {
    if (pts.length < 3) return pts;
    const [a, b] = [pts[0]!, pts[pts.length - 1]!];
    const dx = b[0] - a[0], dy = b[1] - a[1], l = Math.hypot(dx, dy) || 1e-12;
    let worst = 0, k = 0;
    for (let i = 1; i < pts.length - 1; i++) {
      const d = Math.abs((pts[i]![0] - a[0]) * dy - (pts[i]![1] - a[1]) * dx) / l;
      if (d > worst) (worst = d), (k = i);
    }
    return worst <= tol ? [a, b] : [...rdp(pts.slice(0, k + 1)).slice(0, -1), ...rdp(pts.slice(k))];
  };
  let far = 0;
  for (let i = 0; i < ring.length; i++) if (Math.hypot(ring[i]![0] - ring[0]![0], ring[i]![1] - ring[0]![1]) > Math.hypot(ring[far]![0] - ring[0]![0], ring[far]![1] - ring[0]![1])) far = i;
  return [...rdp(ring.slice(0, far + 1)).slice(0, -1), ...rdp([...ring.slice(far), ring[0]!]).slice(0, -1)];
}

/** 🧊️ The evaluated default pose and posed body solids of one scale. */
export function posed(a: J, scale: string): { state: J; worlds: J[] } {
  const probe = structuredClone(a);
  const state = ev.evaluate(probe, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scale) } });
  return { state, worlds: sil.worldSolids(probe, state.bones, Number(scale)) };
}

/** 🖼️ The projected outer outline (world metres of the view) of the solids with the given ids, or undefined. */
export function outlineOf(worlds: J[], ids: Set<string>, view: View, scale: string): P[] | undefined {
  const mine = worlds.filter((w: J) => ids.has(w.id) || ids.has(String(w.id).replace(/#\d+$/, "")));
  if (mine.length === 0) return undefined;
  const cell = (Number(scale) / 1000) * 0.01;
  const tris: P[][] = [];
  let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
  for (const w of mine) {
    const proj = (k: number): P => {
      const q = sil.project(view, w.positions[3 * k], w.positions[3 * k + 1], w.positions[3 * k + 2]);
      minX = Math.min(minX, q.x), maxX = Math.max(maxX, q.x), minY = Math.min(minY, q.y), maxY = Math.max(maxY, q.y);
      return [q.x, q.y];
    };
    for (let t = 0; t < w.indices.length; t += 3) tris.push([proj(w.indices[t]), proj(w.indices[t + 1]), proj(w.indices[t + 2])]);
  }
  (minX -= 2 * cell), (minY -= 2 * cell);
  const W = Math.ceil((maxX - minX) / cell) + 3, H = Math.ceil((maxY - minY) / cell) + 3;
  const mask = new Uint8Array(W * H);
  for (const [p, q, r] of tris as [P, P, P][]) {
    const x0 = Math.max(0, Math.floor((Math.min(p[0], q[0], r[0]) - minX) / cell)), x1 = Math.min(W - 1, Math.ceil((Math.max(p[0], q[0], r[0]) - minX) / cell));
    const y0 = Math.max(0, Math.floor((Math.min(p[1], q[1], r[1]) - minY) / cell)), y1 = Math.min(H - 1, Math.ceil((Math.max(p[1], q[1], r[1]) - minY) / cell));
    const e = (a1: P, b1: P, x: number, y: number): number => (b1[0] - a1[0]) * (y - a1[1]) - (b1[1] - a1[1]) * (x - a1[0]);
    for (let j = y0; j <= y1; j++)
      for (let i = x0; i <= x1; i++) {
        const x = minX + (i + 0.5) * cell, y = minY + (j + 0.5) * cell;
        const d0 = e(p, q, x, y), d1 = e(q, r, x, y), d2 = e(r, p, x, y);
        if ((d0 >= 0 && d1 >= 0 && d2 >= 0) || (d0 <= 0 && d1 <= 0 && d2 <= 0)) mask[j * W + i] = 1;
      }
  }
  const ring = contour(mask, W, H).map(([i, j]) => [minX + i * cell, minY + j * cell] as P);
  if (ring.length < 4) return undefined;
  return simplify(ring.map(([x, y]) => [x + cell / 2, y + cell / 2] as P), (Number(scale) / 1000) * 0.03);
}

/** ↩️ Carries world points of a view back to the rest coordinates of a shape on `bone` (elevation: the bone's side affine; front and plan: the carry at the constant depth `depth`), or undefined where the view sees the bone edge-on. */
export function toRest(a: J, state: J, bone: string, view: View, ring: P[], depth?: number): number[][] | undefined {
  if (view === "elevation") {
    const inv = rig.invert(state.bones[bone].world);
    return ring.map(([x, y]) => {
      const p = rig.apply(inv, x, y);
      return [+p.x.toFixed(4), +p.y.toFixed(4)];
    });
  }
  const rest = rig.restFrames(a.rig)[bone].frame;
  const now = state.bones[bone].frame;
  const v3 = (v: J): number[] => (Array.isArray(v) ? v : [v.x, v.y, v.z]);
  const [no, nu, nv, nw] = [v3(now.o), v3(now.u), v3(now.v), v3(now.w)];
  const [ro, ru, rv, rw] = [v3(rest.o), v3(rest.u), v3(rest.v), v3(rest.w)];
  const M = [0, 1, 2].map((i) => [0, 1, 2].map((j) => nu[i]! * ru[j]! + nv[i]! * rv[j]! + nw[i]! * rw[j]!));
  const plan = view === "plan";
  const [ia, ib, fixed] = plan ? [0, 2, 1] : [2, 1, 0];
  const c = depth ?? ro[fixed]!;
  const a11 = M[ia]![ia]!, a12 = M[ia]![ib]!, a21 = M[ib]![ia]!, a22 = M[ib]![ib]!;
  const det = a11 * a22 - a12 * a21;
  if (Math.abs(det) < 0.05) return undefined;
  return ring.map(([X, Y]) => {
    const tx = X - no[ia]! - M[ia]![fixed]! * (c - ro[fixed]!);
    const ty = Y - no[ib]! - M[ib]![fixed]! * (c - ro[fixed]!);
    return [+(ro[ia]! + (tx * a22 - a12 * ty) / det).toFixed(4), +(ro[ib]! + (a11 * ty - a21 * tx) / det).toFixed(4)];
  });
}

/** 📏️ The constant depth the engine gives a front or plan shape with an elevation twin: the middle of the twin's rest extent (height in plan, forward position in front). */
export function twinDepth(a: J, twin: J, view: View): number {
  const aff = rig.restFrames(a.rig)[twin.bone].affine;
  const pts = twin.points.map((p: number[]) => rig.apply(aff, p[0], p[1]));
  const key = view === "plan" ? pts.map((p: J) => p.y) : pts.map((p: J) => p.x);
  return (Math.min(...key) + Math.max(...key)) / 2;
}

/** ✒️ Re-traces the poly shapes bound to `bones` in the given views at every authored scale from the solids of the same id (plus `extra`); returns what changed. */
export function retrace(a: J, bones: Set<string>, extra: (shape: J, scale: string) => string[] = () => [], views: View[] = ["front", "plan"]): string[] {
  const changed: string[] = [];
  for (const s of SCALES) {
    if (a.representations[s].status !== "authored" || a.volumes[s].status !== "authored") continue;
    const { state, worlds } = posed(a, s);
    const g = a.representations[s].geometry;
    for (const view of views) {
      for (const shape of g[view]) {
        if (!bones.has(shape.bone) || shape.kind !== "poly" || !shape.closed || shape.solid) continue;
        const twin = view === "elevation" ? shape : g.elevation.find((e: J) => e.id === shape.id && e.bone === shape.bone);
        if (!twin) continue;
        const ring = outlineOf(worlds, new Set([shape.id, ...extra(shape, s)]), view, s);
        if (!ring) continue;
        const rest = toRest(a, state, shape.bone, view, ring, view === "elevation" ? undefined : twinDepth(a, twin, view));
        if (!rest) continue;
        shape.points = rest;
        changed.push(`${s}/${view}/${shape.id}`);
      }
    }
  }
  return changed;
}

/** ➕️ A new closed poly shape traced from solids at one scale and view, bound to `bone` (front and plan at `depth`, the depth its elevation twin gives it, else the bone's rest depth). */
export function traced(a: J, scale: string, view: View, ids: string[], bone: string, props: J, depth?: number): J | undefined {
  const { state, worlds } = posed(a, scale);
  const ring = outlineOf(worlds, new Set(ids), view, scale);
  if (!ring) return undefined;
  const points = toRest(a, state, bone, view, ring, depth);
  if (!points) return undefined;
  return { ...props, bone, kind: "poly", closed: true, points };
}
