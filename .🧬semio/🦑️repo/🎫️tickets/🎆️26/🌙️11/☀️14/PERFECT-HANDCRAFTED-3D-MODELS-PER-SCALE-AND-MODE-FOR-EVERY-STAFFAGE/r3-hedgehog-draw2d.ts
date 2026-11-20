/** ✏️ Round 3 hedgehog, ruling E1: redraws the 2D shapes of both authored scales and all three views as the true orthographic view of that scale's body in the foraging stance. Per shape the union silhouette of the body solids assigned to it is traced, rounded, simplified and written in the shape's own frame (solved through the product's placement of a probe); ids, roles, bone bindings, tones, regions and hatches of the old drawing stay where the part still exists, new parts (chest, front dome, forearm, shank, paws in the side view, nose, eye) take them from the sibling named in `NEW`, and the layer order is restated for the new body. A part has the same shape id in all three views, because the product carries a front or plan shape through a pose at the depth of its side-view twin.
 * Usage: bun r3-hedgehog-draw2d.ts   (writes 🗑️generated/r3-hedgehog/reps.json = { representations })
 * @see ./r3-hedgehog-work.ts
 * @see ./r3-domestic-cat-draw2d.ts */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { OUT, working } from "./r3-hedgehog-work.ts";
import { REP_DECISIONS } from "./r3-hedgehog-texts.ts";
import { Mask, featureM, meshMask, meshesAt, poseState, projectWorld, type Json, type SolidMesh, type View } from "./volumes-lib.ts";

type P = [number, number];
const r5 = (x: number): number => Math.round(x * 1e5) / 1e5 + 0;
const nf = (s: string): string => (s === "n" ? "near" : "far");
const LIMB: Record<string, string> = { thigh: "hind", shank: "shank", "foot-h": "hindpaw", "toes-h": "hindpaw", arm: "fore", forearm: "forearm", "foot-f": "forepaw", "toes-f": "forepaw" };

/** 🏷️ The shape a body solid is drawn in at a view (null: not drawn there). */
function shapeOf(view: View, solid: Json): string | null {
  const id: string = solid.id;
  let m: RegExpMatchArray | null;
  if (id === "belly" || id === "tail") return "belly";
  if (id === "chest" || id === "neck") return id;
  if (id === "dome" || id === "saddle" || /^sp-s-/.test(id)) return "dome";
  if (id === "dome-front") return "dome-front";
  if (/^sp-d-/.test(id)) return solid.bone === "trunk" ? "dome" : "dome-front";
  if ((m = id.match(/^(?:bowl|sp)-([rf])([nf])/))) return `skirt-${m[1] === "r" ? "rear" : "front"}-${nf(m[2]!)}`;
  if (id === "head" || id === "jaw" || /^ear-/.test(id)) return "head";
  if (id === "nose") return "nose";
  if (/^eye-[nf]$/.test(id)) return view === "plan" ? null : id;
  if ((m = id.match(/^(thigh|shank|foot-h|toes-h|arm|forearm|foot-f|toes-f)-?([nf])$/))) return `${LIMB[m[1]!]}-${m[2]}`;
  throw new Error(`no shape for solid ${id}`);
}

/** 🎨️ Role, tone, thermal region and hatch of every drawn part (the values the old drawing gave these parts; the nose and the eyes are new details). */
function style(id: string): Json {
  const part = id.replace(/^(near|far)-/, "").replace(/-(n|f|near|far)$/, "");
  if (part === "belly" || part === "chest") return { role: "body", tone: 0.25, region: "belly-fur" };
  if (part === "dome" || part === "dome-front") return { role: "body", tone: 0.35, region: "spines-dome", hatch: "spine-lines" };
  if (part === "skirt-rear" || part === "skirt-front") return { role: "body", tone: 0.3, region: "spines-skirt", hatch: "spine-lines" };
  if (part === "head" || part === "neck") return { role: "body", tone: 0.45, region: "face-fur" };
  if (part === "nose") return { role: "body", tone: 0.1, region: "nose" };
  if (part === "eye") return { role: "body", tone: 0.1, region: "face-fur" };
  if (/paw$/.test(part)) return { role: "body", tone: 0.5, region: "paws-skin" };
  if (/^(hind|shank|fore|forearm)$/.test(part)) return { role: "body", tone: 0.6, region: "limbs-fur" };
  throw new Error(`no style for ${id}`);
}

/** 🪜️ Layer order per view, back to front (elevation: seen from the near side; front: seen from ahead; plan: seen from above). */
function layer(view: View, id: string): number {
  const far = /(^far-)|(-f$)|(-far$)/.test(id);
  const part = id.replace(/^(near|far)-/, "").replace(/-(n|f|near|far)$/, "");
  const hindLeg = /^(hind|shank|hindpaw)$/.test(part);
  const foreLeg = /^(fore|forearm|forepaw)$/.test(part);
  const sub = /paw$/.test(part) ? 0.02 : /^(shank|forearm)$/.test(part) ? 0.01 : 0;
  if (view === "elevation") {
    if (hindLeg || foreLeg) return (far ? 1 : 3.2) + sub;
    if (part === "eye") return far ? 1.95 : 2.1;
    if (far && /^skirt/.test(part)) return 0.5;
    return ({ neck: 1.9, head: 2, nose: 2.1, belly: 3, chest: 3.05, dome: 3.5, "dome-front": 3.55, "skirt-rear": 3.4, "skirt-front": 3.4 } as Record<string, number>)[part]!;
  }
  if (view === "front") {
    if (hindLeg) return 1.1 + sub;
    if (foreLeg) return 3.6 + sub;
    return ({ "skirt-rear": 1, belly: 3, dome: 3.1, chest: 3.3, "skirt-front": 3.45, "dome-front": 3.5, neck: 3.9, head: 4, nose: 4.1, eye: 4.1 } as Record<string, number>)[part]!;
  }
  if (hindLeg || foreLeg) return 1.1 + sub;
  return ({ belly: 1.6, chest: 1.65, neck: 1.9, head: 2, nose: 2.1, "skirt-rear": 3.4, "skirt-front": 3.4, dome: 3.5, "dome-front": 3.55 } as Record<string, number>)[part]!;
}

/** 🧭️ The outer boundary of the largest filled component of a mask as a polygon along the cell edges (world units of the view). */
function contour(mask: Mask): { ring: P[]; parts: number; rings: P[][] } {
  const { width: w, height: h, cells } = mask;
  const at = (i: number, j: number): number => (i < 0 || j < 0 || i >= w || j >= h ? 0 : cells[j * w + i]!);
  const next = new Map<number, number[]>();
  const key = (i: number, j: number): number => j * (w + 1) + i;
  const edge = (a: number, b: number): void => void (next.get(a) ?? next.set(a, []).get(a)!).push(b);
  for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) {
    if (at(i, j) === 0) continue;
    if (at(i, j - 1) === 0) edge(key(i, j), key(i + 1, j));
    if (at(i + 1, j) === 0) edge(key(i + 1, j), key(i + 1, j + 1));
    if (at(i, j + 1) === 0) edge(key(i + 1, j + 1), key(i, j + 1));
    if (at(i - 1, j) === 0) edge(key(i, j + 1), key(i, j));
  }
  const loops: P[][] = [];
  for (const [start] of next) {
    while ((next.get(start)?.length ?? 0) > 0) {
      const loop: P[] = [];
      let a = start;
      let from = -1;
      for (;;) {
        const outs = next.get(a)!;
        let pickAt = 0;
        if (outs.length > 1 && from >= 0) {
          const dx = (a % (w + 1)) - (from % (w + 1));
          const dy = Math.floor(a / (w + 1)) - Math.floor(from / (w + 1));
          let best = -2;
          outs.forEach((b, k) => { const ex = (b % (w + 1)) - (a % (w + 1)); const ey = Math.floor(b / (w + 1)) - Math.floor(a / (w + 1)); const turn = dx * ey - dy * ex; if (turn > best) { best = turn; pickAt = k; } });
        }
        const b = outs.splice(pickAt, 1)[0]!;
        loop.push([mask.box.minX + (a % (w + 1)) * mask.cell, mask.box.minY + Math.floor(a / (w + 1)) * mask.cell]);
        from = a;
        a = b;
        if (a === start) break;
      }
      loops.push(loop);
    }
  }
  const area = (r: P[]): number => r.reduce((s, p, i) => s + p[0] * r[(i + 1) % r.length]![1] - r[(i + 1) % r.length]![0] * p[1], 0) / 2;
  const outer = loops.filter((l) => area(l) > 0).sort((a, b) => area(b) - area(a));
  return { ring: outer[0] ?? [], parts: outer.length, rings: outer };
}

/** ✂️ Douglas-Peucker on a closed ring, split at its two farthest points. */
function simplify(ring: P[], tol: number): P[] {
  const dp = (pts: P[]): P[] => {
    if (pts.length < 3) return pts;
    const [a, b] = [pts[0]!, pts[pts.length - 1]!];
    const len = Math.hypot(b[0] - a[0], b[1] - a[1]) || 1e-12;
    let worst = 0;
    let at = 0;
    for (let i = 1; i < pts.length - 1; i++) {
      const d = Math.abs((b[0] - a[0]) * (a[1] - pts[i]![1]) - (a[0] - pts[i]![0]) * (b[1] - a[1])) / len;
      if (d > worst) { worst = d; at = i; }
    }
    return worst <= tol ? [a, b] : [...dp(pts.slice(0, at + 1)).slice(0, -1), ...dp(pts.slice(at))];
  };
  let i0 = 0;
  for (let i = 1; i < ring.length; i++) if (ring[i]![0] < ring[i0]![0]) i0 = i;
  let i1 = 0;
  let far = -1;
  ring.forEach((p, i) => { const d = Math.hypot(p[0] - ring[i0]![0], p[1] - ring[i0]![1]); if (d > far) { far = d; i1 = i; } });
  const turned = [...ring.slice(i0), ...ring.slice(0, i0)];
  const k = (i1 - i0 + ring.length) % ring.length;
  return [...dp(turned.slice(0, k + 1)).slice(0, -1), ...dp([...turned.slice(k), turned[0]!]).slice(0, -1)];
}

/** ➰️ Chaikin-free rounding of the traced stair: every vertex moves to the mean of itself and its neighbours along the dense ring before simplification. */
function relax(ring: P[], passes: number): P[] {
  let r = ring;
  for (let n = 0; n < passes; n++) r = r.map((p, i) => { const a = r[(i - 1 + r.length) % r.length]!; const b = r[(i + 1) % r.length]!; return [(a[0] + 2 * p[0] + b[0]) / 4, (a[1] + 2 * p[1] + b[1]) / 4] as P; });
  return r;
}

function outline(view: View, meshes: SolidMesh[], feature: number, tol = 0.2, passes = 6): { ring: P[]; parts: number; rings: P[][] } {
  const pts = meshes.flatMap((m) => Array.from({ length: m.world.length / 3 }, (_, i) => projectWorld(view, [m.world[3 * i]!, m.world[3 * i + 1]!, m.world[3 * i + 2]!])));
  const cell = feature / 16;
  const box = { minX: Math.min(...pts.map((p) => p.x)) - 3 * cell, minY: Math.min(...pts.map((p) => p.y)) - 3 * cell, maxX: Math.max(...pts.map((p) => p.x)) + 3 * cell, maxY: Math.max(...pts.map((p) => p.y)) + 3 * cell };
  const mask = meshMask(view, meshes, box, cell);
  const { ring, parts, rings } = contour(mask);
  const clean = (r: P[]): P[] => simplify(relax(r, passes), feature * tol);
  return { ring: clean(ring), parts, rings: rings.filter((r) => r.length * cell * cell > 0 && Math.abs(r.reduce((s, p, i) => s + p[0] * r[(i + 1) % r.length]![1] - r[(i + 1) % r.length]![0] * p[1], 0) / 2) > feature * feature).map(clean) };
}

/** The spiny coat is drawn as a serrated outline whose teeth stand for the tufts: simplified to this share of the smallest feature after this many rounding passes, so that a tooth is a few points and prints under the body pen. */
const SPINY = { tol: 0.34, passes: 14 };
const PROBE: P[] = [[0, 0], [0.1, 0], [0, 0.1], [0.1, 0.1]];
const asset: Json = working(["rig-members"]);
const before: Json = JSON.parse(readFileSync(join(import.meta.dir, "r3-hedgehog-baseline.json"), "utf8")).asset.representations;
const representations: Json = JSON.parse(JSON.stringify(before));
const log: string[] = [];
for (const scale of [50, 100]) {
  const body: Json[] = asset.volumes[String(scale)].body;
  const probeBase: Json = { ...asset, representations: before };
  const meshOf = new Map(meshesAt(probeBase, scale, body).map((m, i) => [body[i]!.id as string, m]));
  const feature = featureM(scale, asset);
  for (const view of ["elevation", "front", "plan"] as View[]) {
    const old: Json[] = before[String(scale)].geometry[view];
    const groups = new Map<string, Json[]>();
    for (const s of body) {
      const id = shapeOf(view, s);
      if (id !== null) (groups.get(id) ?? groups.set(id, []).get(id)!).push(s);
    }
    for (const s of old) if (!groups.has(s.id)) log.push(`1:${scale} ${view}: removed ${s.id} (no body part of its own)`);
    const shapes: Json[] = [...groups.keys()].map((id) => {
      const solids = groups.get(id)!;
      const bones = new Set(solids.map((q) => q.bone));
      if (bones.size !== 1) throw new Error(`1:${scale} ${view} ${id}: solids on several bones ${[...bones]}`);
      if (!old.some((s) => s.id === id)) log.push(`1:${scale} ${view}: added ${id} (bone ${[...bones][0]})`);
      const st = style(id);
      const out: Json = { id, bone: [...bones][0], role: st.role, z: layer(view, id), filled: true, tone: st.tone, region: st.region };
      if (st.hatch !== undefined && view === "elevation") out.hatch = st.hatch;
      out.kind = "poly";
      out.points = PROBE;
      out.closed = true;
      return out;
    }).sort((a, b) => a.z - b.z);
    const probe: Json = { ...asset, representations: { ...before, [String(scale)]: { ...before[String(scale)], geometry: { ...before[String(scale)].geometry, [view]: shapes } } } };
    const state = poseState(probe, scale) as Json;
    const drawn: Json[] = state[view];
    const more: { after: Json; shape: Json }[] = [];
    for (const s of shapes) {
      const ev = drawn.find((d) => d.shape.id === s.id);
      const q: { x: number; y: number }[] = ev?.rings?.[0] ?? [];
      if (q.length < 4) throw new Error(`1:${scale} ${view} ${s.id}: probe gave ${q.length} points`);
      const o = q[0]!;
      const ax: P = [(q[1]!.x - o.x) / 0.1, (q[1]!.y - o.y) / 0.1];
      const ay: P = [(q[2]!.x - o.x) / 0.1, (q[2]!.y - o.y) / 0.1];
      const err = Math.hypot(o.x + 0.1 * ax[0] + 0.1 * ay[0] - q[3]!.x, o.y + 0.1 * ax[1] + 0.1 * ay[1] - q[3]!.y);
      if (err > 1e-6) throw new Error(`1:${scale} ${view} ${s.id}: placement not affine (${err})`);
      const det = ax[0] * ay[1] - ax[1] * ay[0];
      const toLocal = (p: P): P => { const dx = p[0] - o.x; const dy = p[1] - o.y; return [r5((ay[1] * dx - ay[0] * dy) / det), r5((-ax[1] * dx + ax[0] * dy) / det)]; };
      const { ring, rings } = outline(view, groups.get(s.id)!.map((q2) => meshOf.get(q2.id)!), feature, /^(dome|skirt)/.test(s.id) ? SPINY.tol : 0.2, /^(dome|skirt)/.test(s.id) ? SPINY.passes : 6);
      s.points = ring.map(toLocal);
      rings.slice(1).forEach((extra, n) => { more.push({ after: s, shape: { ...s, id: `${s.id}-${"bcdefgh"[n]}`, points: extra.map(toLocal) } }); log.push(`1:${scale} ${view}: ${s.id} has a separate part, drawn as ${s.id}-${"bcdefgh"[n]}`); });
    }
    for (const m of more) shapes.splice(shapes.indexOf(m.after) + 1, 0, m.shape);
    representations[String(scale)].geometry[view] = shapes;
    console.log(`1:${scale} ${view}: ${shapes.length} shapes, ${shapes.reduce((n, s) => n + s.points.length, 0)} points`);
  }
  representations[String(scale)].decisions = { ...representations[String(scale)].decisions, ...REP_DECISIONS[scale] };
}
writeFileSync(join(OUT, "reps.json"), JSON.stringify({ representations }));
writeFileSync(join(OUT, "draw2d-log.txt"), log.join("\n") + "\n");
console.log(log.join("\n"));
