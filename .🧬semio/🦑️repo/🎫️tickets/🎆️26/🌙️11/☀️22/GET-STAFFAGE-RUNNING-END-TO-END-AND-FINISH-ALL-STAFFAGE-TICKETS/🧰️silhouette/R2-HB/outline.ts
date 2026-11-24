/** ✍️ Redraws parts of one view from the volume: per spec the outline of the projected body solids (their exact union, simplified to a fraction of the smallest feature) as a filled shape in the coordinates of its bone, written as `shape-add` ops (an existing shape of the id is replaced in place). The mapping from the view to the shape coordinates is probed through `evaluate` and the placed result is checked against the outline.
 *  bun outline.ts <id|file.json> <scale> <view> <ops-out.json|-> <spec>… [--base ops.json]
 *  spec: <shape id>=<solid id>[+<solid id>…][@key=value,…]; a solid id may hold `*` for any run of characters; keys: after, like (shape whose properties are copied), bone, role, tone, z, region, material, pen, hull (1: convex hull), tol (mm), minus (solid ids joined by `+` whose outline is cut out), axis (1: an open stroke along the long axis of the projected solid at a pen of its width, for a slender bar; a bar seen end-on stays an outline), rm (1: write a shape-remove of the id instead), prism (a share 0..1: the shape becomes a prism on the bone of its first solid, swept along the bone axis nearest to the viewing direction over that share of the depth of the solids, with the outline shrunk along the seen sweep as its profile, so the prism covers the outline in this view and has the depth of the body in every other pose; fit=<mm> takes the deepest prism whose shrunk profile still covers the outline to within that distance; smear=<mm> caps the depth so that the view sees the sweep no longer than that), sheet (1: the shape becomes a plate 0.8 mm thick on the bone of its first solid, swept along the bone axis nearest to the viewing direction, with the outline as its profile: it is carried by the full frame of the bone in every pose), keep (1: the shape keeps the `solid` sweep of the shape it replaces, a thin sheet or a prism seen along its sweep, and the outline becomes its profile) */
import polygonClipping from "polygon-clipping";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { project, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { restFrames } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { worldFrameOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🦴️skeleton/🟦️.ts";
import { applyOps, mergeOps, readOps, writeOps, type Op } from "./ops.ts";
import { fileOf } from "./people.ts";

type P = [number, number];
const args = process.argv.slice(2);
const baseAt = args.indexOf("--base");
const base = baseAt < 0 ? undefined : args.splice(baseAt, 2)[1];
const [name, scaleArg, view, out, ...specs] = args as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
if (base !== undefined) applyOps(asset, await readOps(base));
const scale = Number(scaleArg), feature = featureOf(scale);
const sel = { kind: "scale", scale: { denominator: scale } } as const;
const pose = { kind: "pose", pose: asset.presentation.defaultPose } as const;
const state: any = evaluate(asset, pose, sel);
const worlds = worldSolids(asset, state.bones as never, scale) as any[];
const shapes = asset.representations[scaleArg!].geometry[view!] as any[];
const r5 = (n: number): number => Math.round(n * 1e5) / 1e5;

const pick = (ids: string): any[] => ids.split("+").flatMap((id) => {
  const glob = id.includes("*") ? new RegExp(`^${id.split("*").map((part) => part.replace(/[.+?^${}()|[\]\\]/gu, "\\$&")).join(".*")}$`, "u") : undefined;
  const hit = worlds.filter((w) => w.texture !== true && (glob === undefined ? w.id === id : glob.test(w.id)));
  if (hit.length === 0) throw new Error(`no solid "${id}"`);
  return hit;
});
const unionAt = (list: any[], grid: number, chunk: number): P[][][] => {
  const tris: P[][][] = [];
  for (const w of list) {
    const q: P[] = [];
    for (let i = 0; i < w.positions.length; i += 3) { const p = project(view as never, w.positions[i], w.positions[i + 1], w.positions[i + 2]); q.push([Math.round(p.x * grid) / grid, Math.round(p.y * grid) / grid]); }
    for (let t = 0; t < w.indices.length; t += 3) {
      const [a, b, c] = [q[w.indices[t]]!, q[w.indices[t + 1]]!, q[w.indices[t + 2]]!];
      if (Math.abs((b[0] - a[0]) * (c[1] - a[1]) - (c[0] - a[0]) * (b[1] - a[1])) > 1e-10) tris.push([[a, b, c, a]]);
    }
  }
  let acc: P[][][] = [];
  for (let i = 0; i < tris.length; i += chunk) acc = polygonClipping.union(acc as never, ...(tris.slice(i, i + chunk) as never[])) as never;
  return acc;
};
/** ➕️ The exact union of the projected triangles of the solids; the clipping library fails on some near-degenerate inputs, then the same union is taken on a coarser coordinate grid (down to 0.02 mm) and in smaller batches. */
const union = (list: any[]): P[][][] => {
  let last: unknown;
  for (const [grid, chunk] of [[1e6, 400], [1e5, 400], [1e5, 60], [5e4, 60], [5e4, 12], [2e4, 12]] as const) {
    try { return unionAt(list, grid, chunk); } catch (error) { last = error; }
  }
  throw last;
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
const hullOf = (pts: P[]): P[] => {
  const s = [...pts].sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  const cross = (o: P, a: P, b: P): number => (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
  const half = (list: P[]): P[] => { const h: P[] = []; for (const p of list) { while (h.length >= 2 && cross(h[h.length - 2]!, h[h.length - 1]!, p) <= 0) h.pop(); h.push(p); } h.pop(); return h; };
  return [...half(s), ...half(s.reverse())];
};
const probe = (bone: string, solid?: unknown): ((p: P) => P) => {
  const image = (at: P): P => {
    const copy = structuredClone(asset);
    const e = solid === undefined ? 1 : 1e-4;
    copy.representations[scaleArg!].geometry[view!].push({ id: "probe-outline", bone, role: "body", z: 0, filled: true, tone: 0.5, kind: "poly", points: solid === undefined ? [[0, 0], [1, 0], [0, 1]] : [[at[0] - e, at[1] - e], [at[0] + e, at[1] - e], [at[0] + e, at[1] + e], [at[0] - e, at[1] + e]], closed: true, ...(solid === undefined ? {} : { solid }) });
    const st: any = evaluate(copy, pose, sel);
    const rings = st.shapes[view!].find((s: any) => s.shape.id === "probe-outline").rings as { x: number; y: number }[][];
    if (solid === undefined) return rings[0] as never;
    const pts = rings.flat();
    return [(Math.min(...pts.map((q) => q.x)) + Math.max(...pts.map((q) => q.x))) / 2, (Math.min(...pts.map((q) => q.y)) + Math.max(...pts.map((q) => q.y))) / 2];
  };
  let o: { x: number; y: number }, x: { x: number; y: number }, y: { x: number; y: number };
  if (solid === undefined) [o, x, y] = image([0, 0]) as never;
  else {
    const [po, px, py] = [image([0, 0]), image([0.1, 0]), image([0, 0.1])];
    o = { x: po[0], y: po[1] }; x = { x: po[0] + (px[0] - po[0]) * 10, y: po[1] + (px[1] - po[1]) * 10 }; y = { x: po[0] + (py[0] - po[0]) * 10, y: po[1] + (py[1] - po[1]) * 10 };
  }
  const [a, b, c, d] = [x.x - o.x, y.x - o.x, x.y - o.y, y.y - o.y];
  const det = a * d - b * c;
  return ([wx, wy]) => [r5((d * (wx - o.x) - b * (wy - o.y)) / det), r5((-c * (wx - o.x) + a * (wy - o.y)) / det)];
};

const REST = restFrames(asset.rig) as Record<string, { frame: any }>;
/** 🍃️ The prism a part is drawn as in this view: swept along the axis of the bone that points most towards the viewer, about the middle depth of the solids along that axis; `thick` is the share of the depth of the solids it spans (0: a plate of 0.8 mm). The frame of the bone carries it through every pose. `smear` is the sweep as the view sees it: the drawn prism covers its profile moved along that vector. */
const sheetOf = (bone: string, list: any[], thick: number, cap = Infinity): { solid: { kind: "extruded"; sweep: string; z0: number; z1: number }; smear: P } => {
  const f = state.bones[bone].frame as Record<"o" | "u" | "v" | "w", { x: number; y: number; z: number }>;
  const towards = (a: { x: number; y: number; z: number }): number => Math.abs(view === "elevation" ? a.z : view === "front" ? a.x : a.y);
  const sweep = (["w", "v", "u"] as const).reduce((best, k) => (towards(f[k]) > towards(f[best]) + 1e-9 ? k : best), "w" as "u" | "v" | "w");
  let [lo, hi] = [Infinity, -Infinity];
  for (const w of list) for (let i = 0; i < w.positions.length; i += 3) { const s = (w.positions[i] - f.o.x) * f[sweep].x + (w.positions[i + 1] - f.o.y) * f[sweep].y + (w.positions[i + 2] - f.o.z) * f[sweep].z; [lo, hi] = [Math.min(lo, s), Math.max(hi, s)]; }
  const base = sweep === "w" ? (worldFrameOf(REST[bone]!.frame).o[2] as number) : 0;
  const mid = (lo + hi) / 2 + base;
  const seen = (() => { const [a, b] = [project(view as never, 0, 0, 0), project(view as never, f[sweep].x, f[sweep].y, f[sweep].z)]; return Math.hypot(b.x - a.x, b.y - a.y); })();
  const half = Math.max(0.0004, Math.min((hi - lo) * thick, seen > 1e-9 ? cap / seen : Infinity) / 2);
  const [p0, p1] = [project(view as never, 0, 0, 0), project(view as never, f[sweep].x * 2 * half, f[sweep].y * 2 * half, f[sweep].z * 2 * half)];
  return { solid: { kind: "extruded", sweep, z0: r5(mid - half), z1: r5(mid + half) }, smear: [p1.x - p0.x, p1.y - p0.y] };
};
/** ➖️ The outline shrunk along a vector: the points whose whole segment of that vector, centred on them, lies inside. A prism whose sweep the view sees as that vector and whose profile is the shrunk outline covers the outline again. */
const shrink = (polys: P[][][], s: P): P[][][] => {
  const length = Math.hypot(s[0], s[1]);
  if (length < 2e-4) return polys;
  const n = Math.min(41, Math.max(2, Math.ceil(length / (feature / 6)) + 1));
  let acc = polys;
  for (let k = 0; k < n; k++) {
    const f = k / (n - 1) - 0.5;
    const moved = polys.map((poly) => poly.map((ring) => ring.map(([x, y]) => [x + f * s[0], y + f * s[1]] as P)));
    try { acc = polygonClipping.intersection(acc as never, moved as never) as never; }
    catch { const snap = (source: P[][][]): P[][][] => source.map((poly) => poly.map((ring) => ring.map(([x, y]) => [Math.round(x * 5e4) / 5e4, Math.round(y * 5e4) / 5e4] as P))); try { acc = polygonClipping.intersection(snap(acc) as never, snap(moved) as never) as never; } catch { continue; } }
  }
  return acc;
};
const ops: Op[] = [];
for (const spec of specs) {
  const [head, tail] = spec.split("@") as [string, string | undefined];
  const [id, ids] = head.split("=") as [string, string | undefined];
  const opt = Object.fromEntries((tail ?? "").split(",").filter(Boolean).map((kv) => kv.split("=") as [string, string]));
  if (opt.rm === "1") { ops.push({ op: "shape-remove", scale: scaleArg, view, id }); console.log(`${id}: removed`); continue; }
  const list = pick(ids!);
  const like = shapes.find((s) => s.id === (opt.like ?? id));
  const first = list[0];
  const bone = opt.bone ?? (opt.sheet === "1" || opt.prism !== undefined ? first.bone : like?.bone ?? first.bone);
  const tol = (opt.tol === undefined ? (feature * 1000) / 60 : Number(opt.tol)) / 1000;
  let polys = union(list);
  if (opt.minus !== undefined) polys = polygonClipping.difference(polys as never, union(pick(opt.minus)) as never) as never;
  let rings: P[][] = opt.hull === "1" ? [hullOf(polys.flatMap((poly) => poly[0]!))] : polys.flatMap((poly) => poly.map((ring) => simplify(ring, tol)));
  rings = rings.filter((r) => r.length >= 3 && areaOf(r) > (feature / 20) ** 2);
  const target = polys;
  const cap = opt.smear === undefined ? Infinity : Number(opt.smear) / 1000;
  const coarse = (source: P[][][]): P[][][] => source.map((poly) => poly.map((ring) => { const s = simplify(ring, tol / 2); return [...s, s[0]!]; }).filter((ring) => ring.length >= 4)).filter((poly) => poly.length > 0);
  /** The largest distance by which the prism of a shrunk profile falls short of the outline: the profile moved along the seen sweep covers the outline again except where the outline is narrower than the sweep. */
  const lossOf = (profile: P[][][], s: P): number => {
    const n = Math.min(41, Math.max(2, Math.ceil(Math.hypot(s[0], s[1]) / (feature / 6)) + 1));
    const ringsOf = profile.flatMap((poly) => poly);
    const inside = (q: P): boolean => { let c = false; for (const ring of ringsOf) for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) { const [a, b] = [ring[i]!, ring[j]!]; if ((a[1] > q[1]) !== (b[1] > q[1]) && q[0] < ((b[0] - a[0]) * (q[1] - a[1])) / (b[1] - a[1]) + a[0]) c = !c; } return c; };
    const away = (q: P): number => { let d = Infinity; for (const ring of ringsOf) for (let i = 0; i + 1 < ring.length; i++) { const [a, b] = [ring[i]!, ring[i + 1]!]; const [dx, dy] = [b[0] - a[0], b[1] - a[1]]; const l = dx * dx + dy * dy; const f = l === 0 ? 0 : Math.max(0, Math.min(1, ((q[0] - a[0]) * dx + (q[1] - a[1]) * dy) / l)); d = Math.min(d, Math.hypot(q[0] - a[0] - f * dx, q[1] - a[1] - f * dy)); } return d; };
    let worst = 0;
    for (const poly of target) for (const ring of poly) for (const q of simplify(ring, tol / 2)) {
      let d = Infinity;
      for (let k = 0; k < n && d > 0; k++) { const at: P = [q[0] - (k / (n - 1) - 0.5) * s[0], q[1] - (k / (n - 1) - 0.5) * s[1]]; d = inside(at) ? 0 : Math.min(d, away(at)); }
      worst = Math.max(worst, d);
    }
    return worst;
  };
  let plate = opt.sheet === "1" || opt.prism !== undefined ? sheetOf(bone, list, opt.prism === undefined ? 0 : Number(opt.prism), cap) : undefined;
  let loss = 0;
  if (plate !== undefined && opt.prism !== undefined) {
    const allowed = opt.fit === undefined ? Infinity : Number(opt.fit) / 1000;
    const base = coarse(polys);
    let found: P[][][] | undefined;
    for (const share of opt.fit === undefined ? [1] : [1, 0.7, 0.5, 0.35, 0.25, 0.15, 0.08, 0]) {
      plate = sheetOf(bone, list, Number(opt.prism) * share, cap);
      const profile = shrink(base, plate.smear);
      if (profile.length === 0) continue;
      loss = Math.hypot(plate.smear[0], plate.smear[1]) < 2e-4 ? 0 : lossOf(profile, plate.smear);
      if (loss <= allowed || share === 0) { found = profile; break; }
    }
    if (found === undefined) throw new Error(`${id}: the prism leaves no profile`);
    polys = found;
    rings = polys.flatMap((poly) => poly.map((ring) => simplify(ring, tol))).filter((r) => r.length >= 3 && areaOf(r) > (feature / 20) ** 2);
  }
  const kept = opt.keep === "1" ? like?.solid : plate?.solid;
  const toLocal = probe(bone, kept);
  let stroke: { points: P[]; pen: number } | undefined;
  if (opt.axis === "1") {
    const pts = polys.flatMap((poly) => poly[0]!);
    const [mx, my] = [pts.reduce((s, p) => s + p[0], 0) / pts.length, pts.reduce((s, p) => s + p[1], 0) / pts.length];
    let [sxx, sxy, syy] = [0, 0, 0];
    for (const p of pts) [sxx, sxy, syy] = [sxx + (p[0] - mx) ** 2, sxy + (p[0] - mx) * (p[1] - my), syy + (p[1] - my) ** 2];
    const angle = 0.5 * Math.atan2(2 * sxy, sxx - syy);
    const [ux, uy] = [Math.cos(angle), Math.sin(angle)];
    let [lo, hi, left, right] = [Infinity, -Infinity, Infinity, -Infinity];
    for (const p of pts) { const [a, n] = [(p[0] - mx) * ux + (p[1] - my) * uy, -(p[0] - mx) * uy + (p[1] - my) * ux]; [lo, hi, left, right] = [Math.min(lo, a), Math.max(hi, a), Math.min(left, n), Math.max(right, n)]; }
    const [width, c] = [right - left, (left + right) / 2];
    if (hi - lo > 2 * width) stroke = { points: [[mx - c * uy + ux * (lo + width / 2), my + c * ux + uy * (lo + width / 2)], [mx - c * uy + ux * (hi - width / 2), my + c * ux + uy * (hi - width / 2)]], pen: Math.round(((width * 1000) / scale) * 1000) / 1000 };
  }
  if (stroke !== undefined) rings = [stroke.points];
  const local = rings.map((r) => r.map(toLocal));
  const region = opt.region ?? like?.region ?? (asset.volumes[scaleArg!].thermal as any[]).find((t) => t.of === first.id)?.region;
  const source = (asset.volumes[scaleArg!].body as any[]).find((s) => s.id === first.id);
  const material = opt.material ?? like?.material ?? source.material;
  const shape: any = { id, bone, role: opt.role ?? like?.role ?? (source.role === "detail" ? "detail" : "body"), z: Number(opt.z ?? like?.z ?? 3), filled: true, tone: Number(opt.tone ?? like?.tone ?? source.tone ?? 0.5) };
  if (opt.pen !== undefined) shape.penMm = Number(opt.pen); else if (like?.penMm !== undefined) shape.penMm = like.penMm;
  const own = like !== undefined && opt.like === undefined ? like : undefined;
  if (own?.regions !== undefined && opt.region === undefined) shape.regions = own.regions; else if (region !== undefined) shape.region = region;
  if (own?.hatch !== undefined) shape.hatch = own.hatch;
  if (material !== undefined) shape.material = material;
  if (kept !== undefined) shape.solid = kept;
  if (stroke !== undefined) Object.assign(shape, { filled: false, penMm: Number(opt.pen ?? stroke.pen), kind: "poly", points: local[0], closed: false });
  else if (local.length === 1) Object.assign(shape, { kind: "poly", points: local[0], closed: true });
  else Object.assign(shape, { kind: "path", d: local.map((r) => `M ${r.map((p) => `${p[0]} ${p[1]}`).join(" L ")} Z`).join(" ") });
  const after = opt.after ?? (like === undefined ? shapes[shapes.length - 1]?.id : undefined);
  ops.push(like !== undefined && opt.like === undefined ? { op: "shape-add", scale: scaleArg, view, after: id, shape } : { op: "shape-add", scale: scaleArg, view, ...(after === undefined ? {} : { after }), shape });
  const copy = structuredClone(asset);
  applyOps(copy, [ops[ops.length - 1]!]);
  const placed = (evaluate(copy, pose, sel) as any).shapes[view!].find((s: any) => s.shape.id === id).rings as { x: number; y: number }[][];
  let off = 0;
  if (kept !== undefined) { const b = (pts: { x: number; y: number }[]) => [Math.min(...pts.map((q) => q.x)), Math.max(...pts.map((q) => q.x)), Math.min(...pts.map((q) => q.y)), Math.max(...pts.map((q) => q.y))]; const [u, w] = [b(placed.flat()), b((opt.prism !== undefined ? target.flat(2) : rings.flat()).map((q) => ({ x: q[0], y: q[1] })))]; off = Math.max(...u.map((n, i) => Math.abs(n - w[i]!))); }
  else placed.forEach((ring, i) => ring.forEach((p, j) => { const q = rings[i]?.[j]; if (q !== undefined) off = Math.max(off, Math.hypot(p.x - q[0], p.y - q[1])); }));
  const all = rings.flat();
  console.log(`${id}: ${list.map((w) => w.id).join("+")} on ${bone}${plate === undefined ? "" : ` sweep ${plate.solid.sweep} ${((plate.solid.z1 - plate.solid.z0) * 1000).toFixed(0)} mm seen as ${(Math.hypot(...plate.smear) * 1000).toFixed(1)} mm, short by ${(loss * 1000).toFixed(1)} mm`}, ${rings.length} ring(s) ${rings.map((r) => r.length).join("/")} points, x ${Math.min(...all.map((p) => p[0])).toFixed(4)} ${Math.max(...all.map((p) => p[0])).toFixed(4)} y ${Math.min(...all.map((p) => p[1])).toFixed(4)} ${Math.max(...all.map((p) => p[1])).toFixed(4)}, placed off ${(off * 1000).toFixed(3)} mm`);
}
if (out !== "-") await writeOps(out!, mergeOps(await readOps(out!), ops));
else console.log(ops.map((o) => JSON.stringify(o)).join("\n"));
