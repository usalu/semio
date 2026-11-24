/** 🌿️ The last step of the chain of a plant of R4-P: draws every part of a plant from its body, so that drawing and body are one botanical description per scale. Per scale and view, the body solids of the default pose are grouped by part (the bone, or bone and a capture of `--part` over the solid id). A part is cut into slabs across the bone axis nearest to the viewing direction (`--slab` metres thick, by default 0.6 of the tolerance of the three-views sweep: the larger of 20 mm and 3 % of the largest dimension); the exact union of the projected triangles of a slab is its outline, drawn as a plate 0.004 mm thick on the bone (a thicker plate, seen askew, would smear its outline by up to its thickness) at the middle depth of the slab; neighbouring slabs overlap by a quarter of their thickness, so their plates of one thermal region hide each other's inner outlines and the part reads as one silhouette. Seen in the default pose the plates of a part are exactly its silhouette; in every other pose and clip instant the full frame of the bone carries each plate at its own depth, so the three views stay one object. The plates replace every filled compared shape (body, detail, glazing) the view drew on the bone; strokes stay. Look (tone, region, material, pen) is taken from the shapes they replace (with `--part`: from the replaced shape that lies over the middle of the part, so two tones on one bone stay two tones), the depth order `z` runs linearly with the depth of the slab towards the viewer over the range the view used before, and the texture strokes of a bone are laid on its nearest plate and on those of the adjacent bones of the same thermal region (the two parts of one leaf). Texture and ground shapes stay. With `--strokes` the slender solids are drawn too: every stroke the view drew on a bone that carries slender solids gives way to the centre lines the silhouette guard compares those solids by, as flat open strokes on the bone at a pen of the true width (at least `--pen` mm, by default 0.13), one shape per bone, role and pen, in the depth order of its solids. Parts smaller than `--min` smallest features in both directions of the view are not drawn. The run is idempotent: on a file it has drawn it finds its own plates and writes the same ones.
 *  bun redraw.ts <id> [--in file] [--out file] [--scales 50,100] [--views elevation,front,plan] [--skip <regex of solid ids left as drawn>] [--part <regex with one capture over the solid id>] [--strokes] [--pen <mm>] [--flat (one flat outline per part in the plane of its bone instead of plates with depth: the evaluation thins the profile of a plate to 0.3 mm, a flat path is kept as written, which a foliage full of gaps of a few millimetres needs)] [--digits <decimals of a metre the outlines are written with, default 6; 8 with `--tol 0` keeps a foliage full of small gaps cell-exact>] [--tones (a part per bone and tone of its solids: sunlit and shaded foliage of one bone stay two parts)] [--drop <regex of shape ids removed without a successor>] [--slab <m>] [--min 0] [--tol <share of the smallest feature, default 1/60>]
 * Only the view lists it redraws are rewritten in the text of the file (one shape per line); everything else keeps its bytes.
 * @see ./preserve.ts — the value spans of a JSON text */
import { parseArgs } from "node:util";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { project, strokesOf, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { restFrames } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { worldFrameOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🦴️skeleton/🟦️.ts";
import { fileOf } from "./plants.ts";
import { spans } from "./preserve.ts";

const clipping: any = (await import(Bun.resolveSync("polygon-clipping", "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️catalog-silhouette/🔍️oracles/"))).default;
type P = [number, number];
const { values, positionals } = parseArgs({ allowPositionals: true, options: { in: { type: "string" }, out: { type: "string" }, scales: { type: "string" }, views: { type: "string" }, skip: { type: "string" }, part: { type: "string" }, drop: { type: "string" }, tones: { type: "boolean" }, strokes: { type: "boolean" }, flat: { type: "boolean" }, pen: { type: "string" }, digits: { type: "string" }, slab: { type: "string" }, min: { type: "string" }, tol: { type: "string" } } });
const id = positionals[0]!;
const source = values.in ?? fileOf(id);
const text = await Bun.file(source).text();
const doc = JSON.parse(text);
if (doc.asset.id !== id) throw new Error(`${source} holds ${doc.asset.id}, not ${id}`);
const next = structuredClone(doc);
const asset = next.asset;
const COMPARED = ["body", "detail", "glazing"];
const skip = values.skip === undefined ? undefined : new RegExp(values.skip, "u");
const part = values.part === undefined ? undefined : new RegExp(values.part, "u");
const drop = values.drop === undefined ? undefined : new RegExp(values.drop, "u");
const REST = restFrames(asset.rig) as Record<string, { frame: any }>;
const pose = { kind: "pose", pose: asset.presentation.defaultPose } as const;
const UNIT = 10 ** Number(values.digits ?? 6);
const r6 = (n: number): number => Math.round(n * UNIT) / UNIT;

const unionAt = (tris: P[][][], grid: number, chunk: number): P[][][] => {
  const snapped = tris.map((t) => [t[0]!.map(([x, y]) => [Math.round(x * grid) / grid, Math.round(y * grid) / grid] as P)]).filter(([[a, b, c]]) => Math.abs((b![0] - a![0]) * (c![1] - a![1]) - (c![0] - a![0]) * (b![1] - a![1])) > 1e-15);
  let acc: P[][][] = [];
  for (let i = 0; i < snapped.length; i += chunk) acc = clipping.union(acc, ...snapped.slice(i, i + chunk));
  return acc;
};
/** 🕳️ How many triangles (larger than 0.01 mm2) a union does not cover at their centroid: the clipping library sometimes drops area of a large soup of triangles without failing. */
const missed = (polys: P[][][], tris: P[][][]): number => {
  const boxes = polys.map((poly) => { let [x0, x1, y0, y1] = [Infinity, -Infinity, Infinity, -Infinity]; for (const [x, y] of poly[0]!) [x0, x1, y0, y1] = [Math.min(x0, x), Math.max(x1, x), Math.min(y0, y), Math.max(y1, y)]; return [x0, x1, y0, y1] as const; });
  let count = 0;
  for (const [[a, b, c]] of tris) {
    if (Math.abs((b![0] - a![0]) * (c![1] - a![1]) - (c![0] - a![0]) * (b![1] - a![1])) < 2e-8) continue;
    const [x, y] = [(a![0] + b![0] + c![0]) / 3, (a![1] + b![1] + c![1]) / 3];
    let held = false;
    for (let k = 0; k < polys.length && !held; k++) {
      const box = boxes[k]!;
      if (x < box[0] || x > box[1] || y < box[2] || y > box[3]) continue;
      let odd = false;
      for (const ring of polys[k]!) for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) { const [p, q] = [ring[i]!, ring[j]!]; if (p[1] > y !== q[1] > y && x < ((q[0] - p[0]) * (y - p[1])) / (q[1] - p[1]) + p[0]) odd = !odd; }
      held = odd;
    }
    if (!held) count++;
  }
  return count;
};
let uncovered = 0;
/** ➕️ The exact union of projected triangles. The clipping library fails on some near-degenerate inputs or drops area; the union is then taken on another or a coarser coordinate grid (down to 0.05 mm) and in smaller batches, and the first that covers every triangle is taken (else the one that misses the fewest, counted in `uncovered`). */
const union = (tris: P[][][]): P[][][] => {
  let last: unknown;
  let best: { polys: P[][][]; miss: number } | undefined;
  for (const [grid, chunk] of [...(UNIT > 1e6 ? [[UNIT, 400], [UNIT, 60], [UNIT, 12]] : []), [1e6, 400], [1e6, 60], [1e6, 12], [1e5, 400], [1e5, 60], [9.7e5, 400], [9.1e4, 60], [5e4, 60], [5e4, 12], [4.7e4, 12], [2e4, 12], [2e4, 3], [1.9e4, 3]] as [number, number][]) {
    try {
      const polys = unionAt(tris, grid, chunk);
      const miss = missed(polys, tris);
      if (best === undefined || miss < best.miss) best = { polys, miss };
      if (miss === 0) break;
    } catch (error) { last = error; }
  }
  if (best === undefined) throw last;
  uncovered += best.miss;
  return best.polys;
};
const areaOf = (r: P[]): number => { let s = 0; for (let i = 0; i < r.length; i++) { const [a, b] = [r[i]!, r[(i + 1) % r.length]!]; s += a[0] * b[1] - b[0] * a[1]; } return Math.abs(s / 2); };
const simplify = (ring: P[], tol: number): P[] => {
  const pts = ring.slice(0, -1);
  let far = 0, best = -1;
  for (let i = 1; i < pts.length; i++) { const d = Math.hypot(pts[i]![0] - pts[0]![0], pts[i]![1] - pts[0]![1]); if (d > best) [best, far] = [d, i]; }
  const dp = (list: P[]): P[] => {
    if (list.length < 3) return list;
    const [a, b] = [list[0]!, list[list.length - 1]!];
    const l = Math.hypot(b[0] - a[0], b[1] - a[1]) || 1;
    let [worst, at] = [0, 0];
    for (let i = 1; i < list.length - 1; i++) { const d = Math.abs((b[0] - a[0]) * (list[i]![1] - a[1]) - (b[1] - a[1]) * (list[i]![0] - a[0])) / l; if (d > worst) [worst, at] = [d, i]; }
    return worst <= tol ? [a, b] : [...dp(list.slice(0, at + 1)).slice(0, -1), ...dp(list.slice(at))];
  };
  return [...dp(pts.slice(0, far + 1)).slice(0, -1), ...dp([...pts.slice(far), pts[0]!]).slice(0, -1)];
};
const most = <T>(list: T[]): T | undefined => { const c = new Map<T, number>(); let best: T | undefined; for (const x of list) { c.set(x, (c.get(x) ?? 0) + 1); if (best === undefined || c.get(x)! > c.get(best)!) best = x; } return best; };
const median = (list: number[]): number => [...list].sort((a, b) => a - b)[Math.floor((list.length - 1) / 2)]!;

type V3 = [number, number, number];
/** ✂️ The part of a triangle with `lo <= s <= hi`, `s` being the coordinate along a direction. */
const clip = (tri: V3[], s: (p: V3) => number, lo: number, hi: number): V3[] => {
  let poly = tri;
  for (const [bound, sign] of [[lo, 1], [hi, -1]] as const) {
    const out: V3[] = [];
    for (let i = 0; i < poly.length; i++) {
      const [a, b] = [poly[i]!, poly[(i + 1) % poly.length]!];
      const [da, db] = [(s(a) - bound) * sign, (s(b) - bound) * sign];
      if (da >= 0) out.push(a);
      if (da >= 0 !== db >= 0) { const f = da / (da - db); out.push([a[0] + f * (b[0] - a[0]), a[1] + f * (b[1] - a[1]), a[2] + f * (b[2] - a[2])]); }
    }
    poly = out;
    if (poly.length < 3) return [];
  }
  return poly;
};
const slab = Number(values.slab ?? 0.6 * Math.max(0.02, 0.03 * Math.max(asset.dimensions.heightM, asset.dimensions.widthM, asset.dimensions.depthM)));
let total = 0;
for (const scaleArg of (values.scales ?? "50,100,200,500").split(",")) {
  const rep = asset.representations[scaleArg];
  if (rep?.status !== "authored" || asset.volumes[scaleArg]?.status !== "authored") continue;
  const scale = Number(scaleArg), feature = featureOf(scale);
  const tol = feature * (values.tol === undefined ? 1 / 60 : Number(values.tol));
  const least = Number(values.min ?? 0) * feature;
  const sel = { kind: "scale", scale: { denominator: scale } } as const;
  const state: any = evaluate(asset, pose, sel);
  const worlds = (worldSolids(asset, state.bones as never, scale) as any[]).filter((w) => w.texture !== true && w.slender !== true && !(skip?.test(w.id) ?? false));
  const body = asset.volumes[scaleArg].body as any[];
  const groups = new Map<string, any[]>();
  const toneOf = new Map<string, number>(body.map((s) => [s.id, s.tone]));
  for (const w of worlds) { const key = `${w.bone}\t${values.tones === true && toneOf.get(w.id) !== undefined ? `tone-${Math.round(toneOf.get(w.id)! * 100)}` : part?.exec(w.id)?.[1] ?? ""}`; (groups.get(key) ?? groups.set(key, []).get(key)!).push(w); }
  const slender = values.strokes === true ? (worldSolids(asset, state.bones as never, scale) as any[]).filter((w) => w.texture !== true && w.slender === true && !(skip?.test(w.id) ?? false)) : [];
  const slenderBones = [...new Set(slender.map((w) => w.bone as string))];
  for (const view of (values.views ?? "elevation,front,plan").split(",")) {
    const shapes = rep.geometry[view] as any[];
    const towards = (a: { x: number; y: number; z: number }): number => Math.abs(view === "elevation" ? a.z : view === "front" ? a.x : a.y);
    const nearness = (p: V3): number => (view === "elevation" ? -p[2] : view === "front" ? p[0] : p[1]);
    type Plate = { key: string; bone: string; n: number; of: number; solid: any; tris: P[][][]; depth: number };
    const plates: Plate[] = [];
    const keys = [...groups.keys()];
    for (const [key, list] of groups) {
      const bone = key.split("\t")[0]!;
      const f = state.bones[bone].frame as Record<"o" | "u" | "v" | "w", { x: number; y: number; z: number }>;
      const sweep = (["w", "v", "u"] as const).reduce((best, k) => (towards(f[k]) > towards(f[best]) + 1e-9 ? k : best), "w" as "u" | "v" | "w");
      const along = (p: V3): number => (p[0] - f.o.x) * f[sweep].x + (p[1] - f.o.y) * f[sweep].y + (p[2] - f.o.z) * f[sweep].z;
      const tris: V3[][] = [];
      let [lo, hi] = [Infinity, -Infinity];
      for (const w of list) {
        const at = (i: number): V3 => [w.positions[i * 3], w.positions[i * 3 + 1], w.positions[i * 3 + 2]];
        for (let t = 0; t < w.indices.length; t += 3) { const tri = [at(w.indices[t]), at(w.indices[t + 1]), at(w.indices[t + 2])]; tris.push(tri); for (const q of tri) [lo, hi] = [Math.min(lo, along(q)), Math.max(hi, along(q))]; }
      }
      const of = values.flat === true ? 1 : Math.max(1, Math.ceil((hi - lo - 1e-9) / slab));
      const step = (hi - lo) / of;
      const base = sweep === "w" ? (worldFrameOf(REST[bone]!.frame).o[2] as number) : 0;
      for (let n = 0; n < of; n++) {
        const [s0, s1] = [lo + n * step, lo + (n + 1) * step];
        const flat: P[][][] = [];
        let [sum, count] = [0, 0];
        for (const tri of tris) {
          const piece = of === 1 ? tri : clip(tri, along, s0 - (n === 0 ? 1 : 0.25 * step), s1 + (n === of - 1 ? 1 : 0.25 * step));
          if (piece.length < 3) continue;
          const q = piece.map((v) => { const at = project(view as never, v[0], v[1], v[2]); return [at.x, at.y] as P; });
          for (let i = 1; i + 1 < q.length; i++) flat.push([[q[0]!, q[i]!, q[i + 1]!, q[0]!]]);
          for (const v of piece) { sum += nearness(v); count++; }
        }
        if (count === 0) continue;
        const mid = (s0 + s1) / 2 + base;
        plates.push({ key, bone, n, of, solid: { kind: "extruded", sweep, z0: Math.round((mid - 2e-6) * 1e7) / 1e7, z1: Math.round((mid + 2e-6) * 1e7) / 1e7 }, tris: flat, depth: sum / count });
      }
    }
    const probes: any[] = [];
    plates.forEach((p, i) => { for (const [n, at] of [[0, [0, 0]], [1, [1, 0]], [2, [0, 1]]] as const) probes.push({ id: `probe-${i}-${n}`, bone: p.bone, role: "body", z: 0, filled: true, tone: 0.5, kind: "poly", points: [[at[0] - 1e-4, at[1] - 1e-4], [at[0] + 1e-4, at[1] - 1e-4], [at[0] + 1e-4, at[1] + 1e-4], [at[0] - 1e-4, at[1] + 1e-4]], closed: true, solid: p.solid }); });
    for (const bone of new Set([...slenderBones, ...(values.flat === true ? plates.map((p) => p.bone) : [])])) probes.push({ id: `flat-${bone}`, bone, role: "body", z: 0, filled: true, tone: 0.5, kind: "poly", points: [[0, 0], [1, 0], [0, 1]], closed: true });
    rep.geometry[view] = probes;
    const flat = new Map<string, { x: number; y: number }[]>();
    const probed: any = evaluate(asset, pose, sel);
    rep.geometry[view] = shapes;
    const image = new Map<string, P>();
    for (const s of probed.shapes[view]) { const pts = (s.rings as { x: number; y: number }[][]).flat(); image.set(s.shape.id, [(Math.min(...pts.map((q) => q.x)) + Math.max(...pts.map((q) => q.x))) / 2, (Math.min(...pts.map((q) => q.y)) + Math.max(...pts.map((q) => q.y))) / 2]); }
    for (const s of probed.shapes[view]) if ((s.shape.id as string).startsWith("flat-")) flat.set(s.shape.bone, s.rings[0]);
    const before = shapes.length;
    let [small, rings, points] = [0, 0, 0];
    const drawnBones = new Set(keys.map((k) => k.split("\t")[0]!));
    const replaced = (s: any): boolean => COMPARED.includes(s.role) && drawnBones.has(s.bone) && !(s.filled === false && (s.kind === "path" || (s.kind === "poly" && s.closed !== true)));
    const old = new Map<string, any[]>();
    for (const s of shapes) if (replaced(s)) (old.get(s.bone) ?? old.set(s.bone, []).get(s.bone)!).push(s);
    const ringsOf = new Map<string, { x: number; y: number }[][]>((state.shapes[view] as any[]).map((e) => [e.shape.id, e.rings]));
    const holds = (rings: { x: number; y: number }[][], q: P): boolean => { let c = false; for (const r of rings) for (let i = 0, j = r.length - 1; i < r.length; j = i++) { const [a, b] = [r[i]!, r[j]!]; if (a.y > q[1] !== b.y > q[1] && q[0] < ((b.x - a.x) * (q[1] - a.y)) / (b.y - a.y) + a.x) c = !c; } return c; };
    const zs = shapes.filter(replaced).map((s) => s.z as number);
    const [z0, z1] = zs.length > 0 ? [Math.round(Math.min(...zs) * 1e4) / 1e4, Math.round(Math.max(...zs) * 1e4) / 1e4] : [1, 2];
    const fresh = new Map<string, any[]>();
    const made: { shape: any; depth: number }[] = [];
    plates.forEach((p, i) => {
      const polys = union(p.tris);
      const all = polys.flatMap((poly) => poly[0]!);
      if (all.length === 0) return;
      const [w, h] = [Math.max(...all.map((q) => q[0])) - Math.min(...all.map((q) => q[0])), Math.max(...all.map((q) => q[1])) - Math.min(...all.map((q) => q[1]))];
      const outline = polys.flatMap((poly) => poly.map((ring) => simplify(ring, tol))).filter((r) => r.length >= 3 && areaOf(r) > (feature / 20) ** 2);
      if ((p.of === 1 && w < least && h < least) || outline.length === 0) { small++; return; }
      const [o, x, y] = (values.flat === true ? flat.get(p.bone)!.map((q) => [q.x, q.y]) : [0, 1, 2].map((n) => image.get(`probe-${i}-${n}`)!)) as [P, P, P];
      const [a, b, c, d] = [x[0] - o[0], y[0] - o[0], x[1] - o[1], y[1] - o[1]];
      const det = a * d - b * c;
      if (Math.abs(det) < 1e-9) throw new Error(`${p.key} ${view}: the plate is seen edge-on`);
      const local = outline.map((r) => r.map(([wx, wy]) => [r6((d * (wx - o[0]) - b * (wy - o[1])) / det), r6((-c * (wx - o[0]) + a * (wy - o[1])) / det)] as P));
      const first = body.find((s) => s.id === groups.get(p.key)![0].id) ?? {};
      const tag = p.key.split("\t")[1]!;
      const name = `${p.bone}${tag === "" ? "" : `-${tag}`}-outline${p.of === 1 ? "" : `-${p.n + 1}`}`;
      const mine = old.get(p.bone) ?? [];
      const centre: P = [all.reduce((s, q) => s + q[0], 0) / all.length, all.reduce((s, q) => s + q[1], 0) / all.length];
      const own = mine.filter((s) => s.id === name);
      const over = mine.filter((s) => holds(ringsOf.get(s.id) ?? [], centre)).sort((s, q) => q.z - s.z).slice(0, 1);
      const toned = values.tones === true ? mine.filter((s) => s.tone === first.tone) : [];
      const like = own.length > 0 ? own : toned.length > 0 ? toned : tag !== "" && over.length > 0 ? over : mine;
      const shape: any = { id: name, bone: p.bone, role: most(like.map((s) => s.role as string)) ?? "body", z: 0, filled: true, tone: values.tones === true && first.tone !== undefined ? first.tone : like.length > 0 ? median(like.map((s) => s.tone ?? 0.5)) : first.tone ?? 0.5 };
      const region = most(like.map((s) => s.region).filter((v) => v !== undefined)) ?? (asset.volumes[scaleArg].thermal as any[]).find((t) => t.of === first.id)?.region;
      const material = most(like.map((s) => s.material).filter((v) => v !== undefined)) ?? first.material;
      const pen = most(like.map((s) => s.penMm).filter((v) => v !== undefined));
      if (region !== undefined) shape.region = region;
      if (material !== undefined) shape.material = material;
      if (pen !== undefined && like.every((s) => s.penMm !== undefined)) shape.penMm = pen;
      if (local.length === 1) Object.assign(shape, { kind: "poly", points: local[0], closed: true });
      else Object.assign(shape, { kind: "path", d: local.map((r) => `M ${r.map((q) => `${q[0]} ${q[1]}`).join(" L ")} Z`).join(" ") });
      if (values.flat !== true) shape.solid = p.solid;
      (fresh.get(p.bone) ?? fresh.set(p.bone, []).get(p.bone)!).push(shape);
      made.push({ shape, depth: p.depth });
      rings += local.length; points += local.reduce((s, r) => s + r.length, 0);
    });
    const [d0, d1] = [Math.min(...made.map((m) => m.depth)), Math.max(...made.map((m) => m.depth))];
    for (const m of made) m.shape.z = Math.round((z0 + ((z1 - z0) * (m.depth - d0)) / (d1 - d0 || 1)) * 1e4) / 1e4;
    const own = new Map<string, number>();
    const regionOf = new Map<string, string | undefined>();
    for (const m of made) { own.set(m.shape.bone, Math.max(own.get(m.shape.bone) ?? -Infinity, m.shape.z)); regionOf.set(m.shape.bone, m.shape.region); }
    const top = new Map<string, number>();
    for (const [bone, z] of own) {
      const near = (asset.rig.bones as any[]).filter((b) => (b.id === bone ? false : b.parent === bone || b.id === (asset.rig.bones as any[]).find((x) => x.id === bone)?.parent) && own.has(b.id) && regionOf.get(b.id) !== undefined && regionOf.get(b.id) === regionOf.get(bone));
      top.set(bone, Math.max(z, ...near.map((b) => own.get(b.id)!)));
    }
    const out: any[] = [];
    const placed = new Set<string>();
    for (const s of shapes) {
      if (drop?.test(s.id) ?? false) continue;
      if (!replaced(s)) { out.push(s); continue; }
      const k = s.bone as string;
      if (!placed.has(k)) { placed.add(k); out.push(...(fresh.get(k) ?? [])); }
    }
    for (const [k, list] of fresh) if (!placed.has(k)) out.push(...list);
    let drawnStrokes = 0;
    const strokeShapes = new Set<any>();
    if (values.strokes === true) {
      const stroke = (s: any): boolean => COMPARED.includes(s.role) && slenderBones.includes(s.bone) && s.filled === false && (s.kind === "path" || (s.kind === "poly" && s.closed !== true));
      const minPen = Number(values.pen ?? 0.13);
      const made = new Map<string, any[]>();
      for (const bone of slenderBones) {
        const [o, x, y] = flat.get(bone)!;
        const [a, b, c, d] = [x!.x - o!.x, y!.x - o!.x, x!.y - o!.y, y!.y - o!.y];
        const det = a * d - b * c;
        if (Math.abs(det) < 1e-9) throw new Error(`${bone} ${view}: the plane of the bone is seen edge-on`);
        const local = (wx: number, wy: number): P => [r6((d * (wx - o!.x) - b * (wy - o!.y)) / det), r6((-c * (wx - o!.x) + a * (wy - o!.y)) / det)];
        const sets = new Map<string, { role: string; pen: number; material: string | undefined; tone: number; d: string[]; depth: number; count: number }>();
        for (const w of slender.filter((s) => s.bone === bone)) {
          const source = body.find((s) => s.id === w.id) ?? {};
          const role = rep.pens[source.role] != null && COMPARED.includes(source.role) ? source.role : "body";
          const quads = strokesOf(view as never, w, []) as { x: number; y: number }[][];
          for (const q of quads) {
            const h = Math.hypot(q[0]!.x - q[3]!.x, q[0]!.y - q[3]!.y) / 2;
            const l = Math.hypot(q[1]!.x - q[0]!.x, q[1]!.y - q[0]!.y) || 1;
            const [ex, ey] = [((q[1]!.x - q[0]!.x) / l) * h, ((q[1]!.y - q[0]!.y) / l) * h];
            const [p0, p1] = [local((q[0]!.x + q[3]!.x) / 2 + ex, (q[0]!.y + q[3]!.y) / 2 + ey), local((q[1]!.x + q[2]!.x) / 2 - ex, (q[1]!.y + q[2]!.y) / 2 - ey)];
            const pen = Math.max(minPen, Math.round(((2 * h * 1000) / scale) * 100) / 100);
            const key = `${role}\t${pen}\t${w.material ?? ""}`;
            const set = sets.get(key) ?? sets.set(key, { role, pen, material: w.material, tone: source.tone ?? 0.4, d: [], depth: 0, count: 0 }).get(key)!;
            for (let i = 0; i < w.positions.length; i += 3) { set.depth += nearness([w.positions[i], w.positions[i + 1], w.positions[i + 2]]); set.count++; }
            set.d.push(`M ${p0[0]} ${p0[1]} L ${p1[0]} ${p1[1]}`);
          }
        }
        const olds = out.filter((s) => stroke(s) && s.bone === bone);
        const list: any[] = [];
        [...sets.values()].sort((p, q) => (p.role === q.role ? q.pen - p.pen : p.role < q.role ? -1 : 1)).forEach((set, n) => {
          const like = olds.filter((s) => s.role === set.role);
          const region = most((like.length > 0 ? like : olds).map((s) => s.region).filter((v) => v !== undefined));
          const shape: any = { id: `${bone}-stroke-${n + 1}`, bone, role: set.role, z: Math.round((z0 + ((z1 - z0) * (set.depth / set.count - d0)) / (d1 - d0 || 1)) * 1e4) / 1e4, filled: false, tone: set.tone };
          if (region !== undefined && like.length > 0) shape.region = region;
          if (set.material !== undefined) shape.material = set.material;
          Object.assign(shape, { penMm: set.pen, kind: "path", d: set.d.join(" ") });
          list.push(shape);
          strokeShapes.add(shape);
          drawnStrokes++;
        });
        made.set(bone, list);
      }
      const kept: any[] = [];
      const placed = new Set<string>();
      for (const s of out) {
        if (!stroke(s)) { kept.push(s); continue; }
        if (!placed.has(s.bone)) { placed.add(s.bone); kept.push(...made.get(s.bone)!); }
      }
      for (const [bone, list] of made) if (!placed.has(bone)) kept.push(...list);
      out.length = 0;
      out.push(...kept);
    }
    const lines = new Map<string, any[]>();
    for (const s of out) if (top.has(s.bone) && !strokeShapes.has(s) && !made.some((m) => m.shape === s) && (s.role === "texture" || COMPARED.includes(s.role))) (lines.get(s.bone) ?? lines.set(s.bone, []).get(s.bone)!).push(s);
    for (const [bone, list] of lines) list.sort((p, q) => p.z - q.z).forEach((s, i) => { s.z = Math.round((top.get(bone)! + 1e-4 * (i + 1)) * 1e5) / 1e5; });
    rep.geometry[view] = out;
    total += made.length;
    console.log(`${id} 1:${scale} ${view}: ${before} shapes -> ${out.length} (${made.length} plates of ${groups.size} parts, ${drawnStrokes} stroke shapes, ${rings} rings, ${points} points, ${small} below the limit${uncovered > 0 ? `, ${uncovered} TRIANGLES NOT COVERED so far` : ""})`);
  }
}
const inline = (v: unknown): string => (Array.isArray(v) ? `[${v.map(inline).join(", ")}]` : v !== null && typeof v === "object" ? `{${Object.entries(v).filter(([, x]) => x !== undefined).map(([k, x]) => `${JSON.stringify(k)}: ${inline(x)}`).join(", ")}}` : JSON.stringify(v));
const member = (node: any, key: string): any => node.members.find((m: any) => m.key === key).value;
const root = spans(text);
const edits: { start: number; end: number; text: string }[] = [];
for (const [scaleArg, rep] of Object.entries(next.asset.representations) as [string, any][]) {
  if (rep.status !== "authored") continue;
  for (const view of Object.keys(rep.geometry)) {
    if (JSON.stringify(rep.geometry[view]) === JSON.stringify(doc.asset.representations[scaleArg].geometry[view])) continue;
    const node = member(member(member(member(member(root, "asset"), "representations"), scaleArg), "geometry"), view);
    const line = text.lastIndexOf("\n", node.start) + 1;
    const indent = " ".repeat(text.slice(line).search(/\S/u));
    edits.push({ start: node.start, end: node.end, text: `[\n${(rep.geometry[view] as unknown[]).map((s) => `${indent}  ${inline(s)}`).join(",\n")}\n${indent}]` });
  }
}
let written = text;
for (const e of edits.sort((p, q) => q.start - p.start)) written = written.slice(0, e.start) + e.text + written.slice(e.end);
if (JSON.stringify(JSON.parse(written)) !== JSON.stringify(next)) throw new Error("the rewritten text does not hold the new document");
if (written === text) console.log(`${id}: the file already has these ${total} outlines`);
else { await Bun.write(values.out ?? source, written); console.log(`${id}: ${total} outlines -> ${values.out ?? source} (${text.length} -> ${written.length} characters)`); }
