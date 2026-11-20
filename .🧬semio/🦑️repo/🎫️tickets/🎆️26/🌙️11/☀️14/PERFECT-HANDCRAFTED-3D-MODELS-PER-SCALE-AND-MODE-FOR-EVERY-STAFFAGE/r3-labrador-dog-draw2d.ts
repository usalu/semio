/** ✏️ Round 3 Labrador: redraws the 2D shapes of every scale and view as the true orthographic view of that scale's body (ruling E1). Per shape: the union silhouette of the body solids it stands for is traced, cleaned to an economical outline and written in the frame the product places the shape in (solved through `evaluate`); role, layer and tone of every shape are stated in `look`, pens and hatches of shapes that existed are kept, the detail and texture strokes are restated by hand in world metres of the default pose.
 * Usage: bun r3-labrador-dog-draw2d.ts   (writes 🗑️generated/r3-labrador-dog/reps.json = { representations })
 * @see ./r3-labrador-dog-apply.py */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { REGION_OF } from "./r3-labrador-dog-50-lists.ts";
import { OUT, working } from "./r3-labrador-dog-work.ts";
import { Mask, featureM, meshMask, meshesAt, poseState, projectWorld, type Json, type SolidMesh, type View } from "./volumes-lib.ts";

type P = [number, number];
const r5 = (x: number): number => Math.round(x * 1e5) / 1e5 + 0;

/** 🏷️ The shape a body solid is drawn in at a scale and view (null: hidden by its twin). */
function shapeOf(scale: number, view: View, id: string): string | null {
  const elev = view === "elevation";
  const nf = (s: string): string => (scale === 500 ? s : s === "n" ? "near" : "far");
  let m: RegExpMatchArray | null;
  if (/^(chest|hindbody|croup|neck|head|jaw|tongue|nose-leather|tail-root|tail-mid|tail-tip|tail)$/.test(id)) return id;
  if ((m = id.match(/^eye-([nf])$/))) return elev ? (m[1] === "n" ? "eye" : null) : id;
  if ((m = id.match(/^ear-([nf])$/))) return `${nf(m[1]!)}-ear`;
  if ((m = id.match(/^(toe-pad|pad)-([hf])([nf])/))) return elev ? `${nf(m[3]!)}-${m[2] === "h" ? "hind" : "fore"}-pad` : `${nf(m[3]!)}-${m[2] === "h" ? "hind-paw" : "forepaw"}`;
  if ((m = id.match(/^(foot|toe|paw)-([hf])([nf])/))) return `${nf(m[3]!)}-${m[2] === "h" ? "hind-paw" : "forepaw"}`;
  if ((m = id.match(/^(pastern|carpal-pad)-([nf])$/))) return `${nf(m[2]!)}-wrist`;
  if ((m = id.match(/^(hock|metatarsus)-([nf])$/))) return `${nf(m[2]!)}-hock`;
  if ((m = id.match(/^shoulder-([nf])$/))) return `${nf(m[1]!)}-${scale === 50 ? "scapula" : "shoulder"}`;
  if ((m = id.match(/^upper-arm-([nf])$/))) return `${nf(m[1]!)}-${scale === 50 ? "humerus" : "upper-arm"}`;
  if ((m = id.match(/^(thigh|shank|forearm)-([nf])$/))) return scale === 200 ? `${nf(m[2]!)}-${m[1] === "forearm" ? "fore-forearm" : `hind-${m[1]}`}` : `${nf(m[2]!)}-${m[1]}`;
  if ((m = id.match(/^(hind-lower|fore-upper|fore-lower|hind-thigh|hind-leg|hind-foot|fore-leg|fore-foot)-([nf])$/))) return `${nf(m[2]!)}-${m[1]}`;
  throw new Error(`no shape for solid ${id}`);
}

/** 🎨️ Role, layer and tone of a shape in a view: the far side is drawn first and one tone lighter in the side elevation, the forelegs stand in front of the chest in the front elevation, the legs lie under the trunk in the plan. */
function look(id: string, view: View): { role: string; z: number; tone: number } {
  const far = /^(far|f)-/.test(id);
  const fore = /(scapula|shoulder|humerus|upper-arm|forearm|wrist|forepaw|fore-)/.test(id);
  if (/-pad(-[b-h])?$/.test(id)) return { role: "detail", z: far ? 1.1 : 5.1, tone: 0.12 };
  if (/-ear$/.test(id)) return view === "elevation" ? (far ? { role: "body", z: 2, tone: 0.5 } : { role: "body", z: 4.5, tone: 0.6 }) : { role: "body", z: 4.5, tone: far ? 0.5 : 0.6 };
  if (/^(near|far|n|f)-/.test(id)) {
    if (view === "elevation") return { role: "body", z: far ? 1 : 5, tone: far ? 0.55 : 0.72 };
    if (view === "front") return fore ? { role: "body", z: 3.6, tone: 0.72 } : { role: "body", z: 1, tone: 0.55 };
    return { role: "body", z: fore ? 1.5 : 1, tone: 0.55 };
  }
  if (id.startsWith("tail")) return { role: "body", z: 2.8, tone: 0.66 };
  if (id === "croup") return { role: "body", z: 2.98, tone: 0.72 };
  if (id === "hindbody") return { role: "body", z: 3, tone: 0.72 };
  if (id === "chest") return { role: "body", z: 3.2, tone: 0.72 };
  if (id === "neck") return { role: "body", z: view === "front" ? 3.4 : 3.3, tone: 0.72 };
  if (id === "jaw") return { role: "body", z: 3.9, tone: 0.72 };
  if (id === "tongue") return { role: "body", z: 3.95, tone: 0.42 };
  if (id === "head") return { role: "body", z: 4, tone: 0.72 };
  if (id === "nose-leather") return { role: "body", z: 4.2, tone: 0.08 };
  if (id.startsWith("eye")) return { role: "detail", z: 5, tone: 0.04 };
  throw new Error(`no look for shape ${id}`);
}

/** 🌡️ Regions a shape lists beside those of its own solids, because the scale has merged them into it. */
function merged(scale: number, id: string): string[] {
  if (scale === 100 && id === "jaw") return ["tongue"];
  if (scale === 200 && id === "head") return ["nose-skin", "tongue"];
  if (scale === 200 && id === "jaw") return ["tongue"];
  if (scale === 200 && /-(hind|fore)-lower$/.test(id)) return ["pads"];
  if (scale === 500 && id === "head") return ["ears", "nose-skin", "tongue"];
  if (scale === 500 && /-(hind|fore)-foot$/.test(id)) return ["pads"];
  return [];
}

/** 🖊️ The open detail and texture strokes of the side elevation, stated in world metres of the default pose on the near side and bound to the bone they move with. */
const STROKES: Record<number, { id: string; bone: string; role: string; z: number; tone: number; closed?: boolean; world: P[] }[]> = {
  50: [
    { id: "brow", bone: "head", role: "detail", z: 5, tone: 0.2, world: [[0.693, 0.676], [0.71, 0.679], [0.727, 0.668]] },
    { id: "mouth-line", bone: "head", role: "detail", z: 5, tone: 0.2, world: [[0.842, 0.585], [0.8, 0.584], [0.74, 0.591], [0.704, 0.603]] },
    { id: "near-hind-toes", bone: "paw-hn", role: "detail", z: 5.2, tone: 0.2, world: [[-0.066, 0.04], [-0.05, 0.03], [-0.042, 0.012]] },
    { id: "near-fore-toes", bone: "paw-fn", role: "detail", z: 5.2, tone: 0.2, world: [[0.449, 0.041], [0.467, 0.03], [0.476, 0.012]] },
    { id: "fur-neck", bone: "neck", role: "texture", z: 5.5, tone: 0.35, world: [[0.535, 0.6], [0.548, 0.578], [0.566, 0.562]] },
    { id: "fur-trunk-1", bone: "thorax", role: "texture", z: 5.5, tone: 0.35, world: [[0.25, 0.345], [0.275, 0.33], [0.305, 0.322]] },
    { id: "fur-trunk-2", bone: "thorax", role: "texture", z: 5.5, tone: 0.35, world: [[0.3, 0.52], [0.33, 0.526], [0.36, 0.536]] },
    { id: "fur-tail", bone: "tail-2", role: "texture", z: 5.5, tone: 0.35, world: [[-0.23, 0.497], [-0.275, 0.5]] },
  ],
  100: [{ id: "eye", bone: "head", role: "detail", z: 5, tone: 0.04, closed: true, world: [[0.7065, 0.6598], [0.7115, 0.6672], [0.7205, 0.6672], [0.7255, 0.6598], [0.7205, 0.6524], [0.7115, 0.6524]] }],
  200: [],
  500: [],
};

/** 🧭️ The outer boundaries of the filled components of a mask as polygons along the cell edges (world units of the view), largest first. */
function contour(mask: Mask): P[][] {
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
  return loops.filter((l) => area(l) > 0).sort((a, b) => area(b) - area(a));
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

/** ➰️ Rounds the traced stair: every vertex moves to the mean of itself and its neighbours along the dense ring before simplification. */
function relax(ring: P[], passes: number): P[] {
  let r = ring;
  for (let n = 0; n < passes; n++) r = r.map((p, i) => { const a = r[(i - 1 + r.length) % r.length]!; const b = r[(i + 1) % r.length]!; return [(a[0] + 2 * p[0] + b[0]) / 4, (a[1] + 2 * p[1] + b[1]) / 4] as P; });
  return r;
}

/** 🫧️ Grows (`fill` 1) or shrinks (`fill` 0) the filled cells of a mask by `n` cells along the four neighbours. */
function grow(mask: Mask, n: number, fill: 0 | 1): void {
  const { width: w, height: h } = mask;
  for (let k = 0; k < n; k++) {
    const src = mask.cells.slice();
    for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) {
      const o = j * w + i;
      if ((src[o]! !== 0) === (fill === 1)) continue;
      const hit = (i > 0 && (src[o - 1]! !== 0) === (fill === 1)) || (i < w - 1 && (src[o + 1]! !== 0) === (fill === 1)) || (j > 0 && (src[o - w]! !== 0) === (fill === 1)) || (j < h - 1 && (src[o + w]! !== 0) === (fill === 1));
      if (hit) mask.cells[o] = fill;
    }
  }
}

/** 🖋️ The outlines of the projected solids of one shape; `closeM` first closes gaps narrower than twice that distance, so pads that stand apart by less than a printable gap are drawn as one mark. */
function outline(view: View, meshes: SolidMesh[], feature: number, closeM = 0): P[][] {
  const pts = meshes.flatMap((m) => Array.from({ length: m.world.length / 3 }, (_, i) => projectWorld(view, [m.world[3 * i]!, m.world[3 * i + 1]!, m.world[3 * i + 2]!])));
  const cell = feature / 16;
  const n = Math.ceil(closeM / cell);
  const pad = (3 + n) * cell;
  const box = { minX: Math.min(...pts.map((p) => p.x)) - pad, minY: Math.min(...pts.map((p) => p.y)) - pad, maxX: Math.max(...pts.map((p) => p.x)) + pad, maxY: Math.max(...pts.map((p) => p.y)) + pad };
  const mask = meshMask(view, meshes, box, cell);
  if (n > 0) {
    grow(mask, n, 1);
    grow(mask, n, 0);
  }
  const area = (r: P[]): number => Math.abs(r.reduce((s, p, i) => s + p[0] * r[(i + 1) % r.length]![1] - r[(i + 1) % r.length]![0] * p[1], 0) / 2);
  return contour(mask).filter((r, i) => i === 0 || area(r) > feature * feature * 0.5).map((r) => simplify(relax(r, 6), feature * 0.2));
}

const PROBE: P[] = [[0, 0], [0.1, 0], [0, 0.1], [0.1, 0.1]];
const asset: Json = working(["roots"]);
const representations: Json = JSON.parse(JSON.stringify(asset.representations));
const log: string[] = [];
for (const scale of [50, 100, 200, 500]) {
  const body: Json[] = asset.volumes[String(scale)].body;
  const meshOf = new Map(meshesAt(asset, scale, body).map((m, i) => [body[i]!.id as string, m]));
  const feature = featureM(scale, asset);
  for (const view of ["elevation", "front", "plan"] as View[]) {
    const old: Json[] = asset.representations[String(scale)].geometry[view];
    const groups = new Map<string, Json[]>();
    for (const s of body) {
      const id = shapeOf(scale, view, s.id);
      if (id !== null) (groups.get(id) ?? groups.set(id, []).get(id)!).push(s);
    }
    const strokes = view === "elevation" ? STROKES[scale]! : [];
    for (const s of old) if (!groups.has(s.id) && !strokes.some((q) => q.id === s.id)) log.push(`1:${scale} ${view}: removed ${s.id}`);
    for (const id of groups.keys()) if (!old.some((s) => s.id === id)) log.push(`1:${scale} ${view}: added ${id}`);
    const shapes: Json[] = [];
    for (const [id, solids] of groups) {
      const bones = new Set(solids.map((q) => q.bone));
      if (bones.size !== 1) throw new Error(`1:${scale} ${view} ${id}: solids on several bones ${[...bones]}`);
      const was = old.find((s) => s.id === id);
      const regions = [...new Set([...solids.map((q) => REGION_OF(q.id)[0]), ...merged(scale, id)])];
      const l = look(id, view);
      const out: Json = { id, bone: [...bones][0], role: l.role, z: l.z, filled: true, tone: l.tone };
      if (!id.startsWith("eye")) Object.assign(out, regions.length === 1 ? { region: regions[0] } : { regions });
      if (was?.penMm !== undefined) out.penMm = was.penMm;
      out.kind = "poly";
      out.points = PROBE;
      out.closed = true;
      if (view !== "elevation" && /^(near|far|n|f)-/.test(id) && !id.endsWith("-ear")) out.followTip = true;
      shapes.push(out);
    }
    for (const st of strokes) shapes.push({ id: st.id, bone: st.bone, role: st.role, z: st.z, filled: st.closed === true, tone: st.tone, kind: "poly", points: PROBE, closed: st.closed === true });
    shapes.sort((a, b) => a.z - b.z);
    const rep = representations[String(scale)];
    const probe: Json = { ...asset, representations: { ...representations, [String(scale)]: { ...rep, geometry: { ...rep.geometry, [view]: shapes } } } };
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
      const solids = groups.get(s.id);
      if (solids === undefined) {
        s.points = strokes.find((st) => st.id === s.id)!.world.map(toLocal);
        continue;
      }
      const rings = outline(view, solids.map((q2) => meshOf.get(q2.id)!), feature, s.id.endsWith("-pad") ? 0.007 : 0);
      s.points = rings[0]!.map(toLocal);
      rings.slice(1).forEach((extra, n) => more.push({ after: s, shape: { ...s, id: `${s.id}-${"bcdefgh"[n]}`, points: extra.map(toLocal) } }));
    }
    for (const m of more) shapes.splice(shapes.indexOf(m.after) + 1, 0, m.shape);
    rep.geometry[view] = shapes;
    console.log(`1:${scale} ${view}: ${shapes.length} shapes, ${shapes.reduce((n, s) => n + s.points.length, 0)} points`);
  }
}
const decisions: Json = JSON.parse(readFileSync(join(import.meta.dir, "r3-labrador-dog-rep-decisions.json"), "utf8"));
for (const scale of [50, 100, 200, 500]) Object.assign(representations[String(scale)].decisions, decisions[String(scale)]);
mkdirSync(OUT, { recursive: true });
writeFileSync(join(OUT, "reps.json"), JSON.stringify({ representations }));
writeFileSync(join(OUT, "draw2d-log.txt"), log.join("\n") + "\n");
console.log(log.length, "changes of the shape lists, see draw2d-log.txt");
