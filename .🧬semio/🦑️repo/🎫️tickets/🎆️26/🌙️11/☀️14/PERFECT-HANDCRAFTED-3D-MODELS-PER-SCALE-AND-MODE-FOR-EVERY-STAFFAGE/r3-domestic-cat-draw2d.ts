/** ✏️ Round 3 domestic cat, ruling E1: redraws the 2D shapes of every authored scale and view as the true orthographic view of that scale's body in the default pose. Per shape: the union silhouette of the body solids assigned to it is traced, rounded, simplified to a fifth of the scale's smallest feature and written in the shape's own frame (solved through the product's placement of a probe); ids, roles, bone bindings, layer order, tones and regions of the drawing stay, a shape is added where a bone had none (rump, tail pieces of the 1:200 front view), front tail columns without a body part of their own go, and the detail and texture strokes are restated by hand on the new form.
 * Usage: bun r3-domestic-cat-draw2d.ts   (writes 🗑️generated/r3-domestic-cat/reps.json = { representations })
 * @see ./r3-domestic-cat-work.ts
 * @see ./r3-goat-draw2d.ts */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { regionOf } from "./r3-domestic-cat-50.ts";
import { OUT, working } from "./r3-domestic-cat-work.ts";
import { Mask, featureM, meshMask, meshesAt, poseState, projectWorld, type Json, type SolidMesh, type View } from "./volumes-lib.ts";

type P = [number, number];
const r5 = (x: number): number => Math.round(x * 1e5) / 1e5 + 0;
const nf = (s: string): string => (s === "n" ? "near" : "far");

/** 🏷️ The shape a body solid is drawn in at a scale and view (null: not drawn there). */
function shapeOf(scale: number, view: View, id: string): string | null {
  const elev = view === "elevation";
  let m: RegExpMatchArray | null;
  if (/^(rump|hindbody|midbody|chest|neck|head|jaw)$/.test(id)) return id;
  if (/^muzzle-[nf]$/.test(id)) return "head";
  if (id === "nose") return scale === 50 ? (elev ? "nose-leather" : "nose") : view === "plan" ? "nose" : "head";
  if (id === "tongue") return elev ? "tongue" : null;
  if ((m = id.match(/^eye-([nf])$/))) return elev ? (m[1] === "n" ? "eye" : null) : scale === 50 ? `${nf(m[1]!)}-eye` : "head";
  if ((m = id.match(/^ear-([nf])$/))) return `${nf(m[1]!)}-ear`;
  if (/^tail-/.test(id)) return id;
  if ((m = id.match(/^(thigh|shank|hock|paw|toe|pad|toepad|sole|lower|foot)-h([nf])/))) {
    const side = scale === 200 && elev ? m[2]! : nf(m[2]!);
    if (scale === 200) return `${side}-hind-${m[1] === "thigh" ? "thigh" : m[1] === "lower" ? "lower" : "foot"}`;
    const part = m[1]!;
    return `${side}-${part === "paw" || part === "toe" ? "hind-paw" : /pad|sole/.test(part) ? (elev ? "hind-pad" : "hind-paw") : part}`;
  }
  if ((m = id.match(/^(shoulder|arm|forearm|wrist|paw|toe|pad|toepad|sole|upper|lower|foot)-f([nf])/))) {
    const side = scale === 200 && elev ? m[2]! : nf(m[2]!);
    const part = m[1]!;
    if (scale === 200) return `${side}-fore-${part === "shoulder" || part === "upper" ? "upper" : part === "lower" ? "lower" : "foot"}`;
    return `${side}-${part === "shoulder" ? "scapula" : part === "arm" ? "humerus" : part === "paw" || part === "toe" ? "fore-paw" : /pad|sole/.test(part) ? (elev ? "fore-pad" : "fore-paw") : part}`;
  }
  throw new Error(`no shape for solid ${id}`);
}

/** 🧬️ The sibling a new shape takes role, layer and tone from, with what differs. */
function template(id: string): { from: string; set: Json } {
  if (id === "rump") return { from: "hindbody", set: { z: 2.95 } };
  if (id === "tail-a" || id === "tail-b") return { from: "hindbody", set: { z: 2.8, tone: 0.66 } };
  throw new Error(`no template rule for ${id}`);
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

function outline(view: View, meshes: SolidMesh[], feature: number): { ring: P[]; parts: number; rings: P[][] } {
  const pts = meshes.flatMap((m) => Array.from({ length: m.world.length / 3 }, (_, i) => projectWorld(view, [m.world[3 * i]!, m.world[3 * i + 1]!, m.world[3 * i + 2]!])));
  const cell = feature / 16;
  const box = { minX: Math.min(...pts.map((p) => p.x)) - 3 * cell, minY: Math.min(...pts.map((p) => p.y)) - 3 * cell, maxX: Math.max(...pts.map((p) => p.x)) + 3 * cell, maxY: Math.max(...pts.map((p) => p.y)) + 3 * cell };
  const mask = meshMask(view, meshes, box, cell);
  const { ring, parts, rings } = contour(mask);
  const clean = (r: P[]): P[] => simplify(relax(r, 6), feature * 0.2);
  return { ring: clean(ring), parts, rings: rings.filter((r) => r.length * cell * cell > 0 && Math.abs(r.reduce((s, p, i) => s + p[0] * r[(i + 1) % r.length]![1] - r[(i + 1) % r.length]![0] * p[1], 0) / 2) > feature * feature).map(clean) };
}

/** Detail and texture strokes restated by hand: bone-local points (`local`, along the bone and across it) or world points of the default pose (`world`). */
const STROKES: Record<string, { bone: string; local?: P[]; world?: P[] }> = {
  brow: { bone: "head", local: [[0.05, 0.027], [0.06, 0.0245], [0.068, 0.018]] },
  "mouth-line": { bone: "head", local: [[0.088, -0.0185], [0.07, -0.0205], [0.05, -0.019]] },
  "inner-ear": { bone: "ear-n", local: [[0.006, -0.005], [0.038, 0.0], [0.008, 0.007]] },
  "near-hind-toes": { bone: "paw-hn", local: [[0.02, 0.006], [0.03, 0.001], [0.033, -0.005]] },
  "near-fore-toes": { bone: "paw-fn", local: [[0.017, 0.006], [0.027, 0.001], [0.03, -0.005]] },
  "tail-ring": { bone: "tail-3", local: [[0.03, -0.0115], [0.03, 0.0115]] },
  "shoulder-line": { bone: "thorax-b", world: [[0.238, 0.236], [0.229, 0.2], [0.224, 0.166]] },
  "hip-line": { bone: "lumbar", world: [[0.034, 0.243], [0.052, 0.2], [0.056, 0.168]] },
  "stripe-1": { bone: "thorax-a", world: [[0.2, 0.257], [0.196, 0.22], [0.2, 0.186]] },
  "stripe-2": { bone: "thorax-a", world: [[0.228, 0.253], [0.224, 0.215], [0.228, 0.181]] },
  "stripe-3": { bone: "lumbar", world: [[0.12, 0.262], [0.116, 0.225], [0.12, 0.191]] },
};
const KEEP_FILL = new Set(["tongue", "eye", "nose-leather"]);
const PROBE: P[] = [[0, 0], [0.1, 0], [0, 0.1], [0.1, 0.1]];

const asset: Json = working(["rig-members"]);
const before: Json = JSON.parse(readFileSync(join(OUT, "asset-before.json"), "utf8")).asset.representations;
const representations: Json = JSON.parse(JSON.stringify(before));
const log: string[] = [];
for (const scale of [50, 100, 200]) {
  const body: Json[] = asset.volumes[String(scale)].body;
  const probeBase: Json = { ...asset, representations: before };
  const meshOf = new Map(meshesAt(probeBase, scale, body).map((m, i) => [body[i]!.id as string, m]));
  const feature = featureM(scale, asset);
  for (const view of ["elevation", "front", "plan"] as View[]) {
    const old: Json[] = before[String(scale)].geometry[view];
    const groups = new Map<string, Json[]>();
    for (const s of body) {
      const id = shapeOf(scale, view, s.id);
      if (id !== null) (groups.get(id) ?? groups.set(id, []).get(id)!).push(s);
    }
    const list: Json[] = old.filter((s) => groups.has(s.id) || (s.id in STROKES && view === "elevation"));
    for (const s of old) if (!list.includes(s)) log.push(`1:${scale} ${view}: removed ${s.id} (no body part of its own)`);
    for (const id of groups.keys()) {
      if (list.some((s) => s.id === id)) continue;
      const t = template(id);
      const src = list.find((s) => s.id === t.from)!;
      const made: Json = { ...src, id, ...t.set };
      delete made.penMm;
      delete made.regions;
      list.splice(list.indexOf(src), 0, made);
      log.push(`1:${scale} ${view}: added ${id} (bone ${groups.get(id)![0]!.bone}) before ${t.from}`);
    }
    const shapes: Json[] = list.map((s) => {
      const solids = groups.get(s.id);
      const out: Json = {};
      for (const k of ["id", "bone", "role", "z", "filled", "tone", "region", "regions", "penMm"]) if (s[k] !== undefined) out[k] = s[k];
      if (solids !== undefined) {
        const bones = new Set(solids.map((q) => q.bone));
        if (bones.size !== 1) throw new Error(`1:${scale} ${view} ${s.id}: solids on several bones ${[...bones]}`);
        const bone = [...bones][0] as string;
        if (out.bone !== bone) { log.push(`1:${scale} ${view}: ${s.id} rebound ${out.bone} -> ${bone}`); out.bone = bone; }
        const isNew = !old.some((o) => o.id === s.id);
        if (isNew) out.region = regionOf(solids[0]!.id)[1];
        if (!KEEP_FILL.has(s.id)) out.filled = true;
      }
      out.kind = "poly";
      if (s.followTip !== undefined) out.followTip = s.followTip;
      out.points = PROBE;
      out.closed = solids !== undefined;
      return out;
    });
    const probe: Json = { ...asset, representations: { ...before, [String(scale)]: { ...before[String(scale)], geometry: { ...before[String(scale)].geometry, [view]: shapes } } } };
    const state = poseState(probe, scale) as Json;
    const drawn: Json[] = state[view];
    const frames = state.frames;
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
      const solids = groups.get(s.id);
      if (solids === undefined) {
        const st = STROKES[s.id]!;
        const f = frames[st.bone]!;
        const world: [number, number, number][] = st.world !== undefined ? st.world.map(([x, y]) => [x, y, 0]) : st.local!.map(([u, v]) => [f.o[0] + f.u[0] * u + f.v[0] * v, f.o[1] + f.u[1] * u + f.v[1] * v, f.o[2] + f.u[2] * u + f.v[2] * v]);
        s.bone = st.bone;
        s.points = world.map((w3) => { const w = projectWorld(view, w3); return toLocal([w.x, w.y]); });
        continue;
      }
      const { ring, rings } = outline(view, solids.map((q2) => meshOf.get(q2.id)!), feature);
      s.points = ring.map(toLocal);
      rings.slice(1).forEach((extra, n) => { more.push({ after: s, shape: { ...s, id: `${s.id}-${"bcdefgh"[n]}`, points: extra.map(toLocal) } }); log.push(`1:${scale} ${view}: ${s.id} has a separate part, drawn as ${s.id}-${"bcdefgh"[n]}`); });
    }
    for (const m of more) shapes.splice(shapes.indexOf(m.after) + 1, 0, m.shape);
    representations[String(scale)].geometry[view] = shapes;
    console.log(`1:${scale} ${view}: ${shapes.length} shapes, ${shapes.reduce((n, s) => n + s.points.length, 0)} points`);
  }
}
const decisions: Json = JSON.parse(readFileSync(join(import.meta.dir, "r3-domestic-cat-rep-decisions.json"), "utf8"));
for (const scale of [50, 100, 200]) representations[String(scale)].decisions = { ...representations[String(scale)].decisions, ...decisions[String(scale)] };
writeFileSync(join(OUT, "reps.json"), JSON.stringify({ representations }));
writeFileSync(join(OUT, "draw2d-log.txt"), log.join("\n") + "\n");
console.log(log.join("\n"));
