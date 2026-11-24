/** ✍️ Redraws parts of one view from the volume: per spec the outline of the projected body solids (their exact union, simplified to a fraction of the smallest feature) as a filled shape in the coordinates of its bone, written as `shape-add` ops (an existing shape of the id is replaced in place). The mapping from the view to the shape coordinates is probed through `evaluate` and the placed result is checked against the outline.
 *  bun outline.ts <id|file.json> <scale> <view> <ops-out.json|-> <spec>… [--base ops.json]
 *  spec: <shape id>=<solid id>[+<solid id>…][@key=value,…]; a solid id may end in `*`; keys: after, like (shape whose properties are copied), bone, role, tone, z, region, material, pen, hull (1: convex hull), tol (mm), minus (solid ids joined by `+` whose outline is cut out), axis (1: an open stroke along the long axis of the projected solid at a pen of its width, for a slender bar; a bar seen end-on stays an outline), rm (1: write a shape-remove of the id instead) */
import polygonClipping from "polygon-clipping";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { project, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
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
  const hit = worlds.filter((w) => (id.endsWith("*") ? w.id.startsWith(id.slice(0, -1)) : w.id === id));
  if (hit.length === 0) throw new Error(`no solid "${id}"`);
  return hit;
});
const union = (list: any[]): P[][][] => {
  const tris: P[][][] = [];
  for (const w of list) {
    const q: P[] = [];
    for (let i = 0; i < w.positions.length; i += 3) { const p = project(view as never, w.positions[i], w.positions[i + 1], w.positions[i + 2]); q.push([Math.round(p.x * 1e6) / 1e6, Math.round(p.y * 1e6) / 1e6]); }
    for (let t = 0; t < w.indices.length; t += 3) {
      const [a, b, c] = [q[w.indices[t]]!, q[w.indices[t + 1]]!, q[w.indices[t + 2]]!];
      if (Math.abs((b[0] - a[0]) * (c[1] - a[1]) - (c[0] - a[0]) * (b[1] - a[1])) > 1e-10) tris.push([[a, b, c, a]]);
    }
  }
  let acc: P[][][] = [];
  for (let i = 0; i < tris.length; i += 400) acc = polygonClipping.union(acc as never, ...(tris.slice(i, i + 400) as never[])) as never;
  return acc;
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
const probe = (bone: string): ((p: P) => P) => {
  const copy = structuredClone(asset);
  copy.representations[scaleArg!].geometry[view!].push({ id: "probe-outline", bone, role: "body", z: 0, filled: true, tone: 0.5, kind: "poly", points: [[0, 0], [1, 0], [0, 1]], closed: true });
  const st: any = evaluate(copy, pose, sel);
  const [o, x, y] = st.shapes[view!].find((s: any) => s.shape.id === "probe-outline").rings[0];
  const [a, b, c, d] = [x.x - o.x, y.x - o.x, x.y - o.y, y.y - o.y];
  const det = a * d - b * c;
  return ([wx, wy]) => [r5((d * (wx - o.x) - b * (wy - o.y)) / det), r5((-c * (wx - o.x) + a * (wy - o.y)) / det)];
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
  const bone = opt.bone ?? like?.bone ?? first.bone;
  const tol = (opt.tol === undefined ? (feature * 1000) / 60 : Number(opt.tol)) / 1000;
  let polys = union(list);
  if (opt.minus !== undefined) polys = polygonClipping.difference(polys as never, union(pick(opt.minus)) as never) as never;
  let rings: P[][] = opt.hull === "1" ? [hullOf(polys.flatMap((poly) => poly[0]!))] : polys.flatMap((poly) => poly.map((ring) => simplify(ring, tol)));
  rings = rings.filter((r) => r.length >= 3 && areaOf(r) > (feature / 20) ** 2);
  const toLocal = probe(bone);
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
  if (region !== undefined) shape.region = region;
  if (material !== undefined) shape.material = material;
  if (stroke !== undefined) Object.assign(shape, { filled: false, penMm: Number(opt.pen ?? stroke.pen), kind: "poly", points: local[0], closed: false });
  else if (local.length === 1) Object.assign(shape, { kind: "poly", points: local[0], closed: true });
  else Object.assign(shape, { kind: "path", d: local.map((r) => `M ${r.map((p) => `${p[0]} ${p[1]}`).join(" L ")} Z`).join(" ") });
  const after = opt.after ?? (like === undefined ? shapes[shapes.length - 1]?.id : undefined);
  ops.push(like !== undefined && opt.like === undefined ? { op: "shape-add", scale: scaleArg, view, after: id, shape } : { op: "shape-add", scale: scaleArg, view, ...(after === undefined ? {} : { after }), shape });
  const copy = structuredClone(asset);
  applyOps(copy, [ops[ops.length - 1]!]);
  const placed = (evaluate(copy, pose, sel) as any).shapes[view!].find((s: any) => s.shape.id === id).rings as { x: number; y: number }[][];
  let off = 0;
  placed.forEach((ring, i) => ring.forEach((p, j) => { const q = rings[i]?.[j]; if (q !== undefined) off = Math.max(off, Math.hypot(p.x - q[0], p.y - q[1])); }));
  const all = rings.flat();
  console.log(`${id}: ${list.map((w) => w.id).join("+")} on ${bone}, ${rings.length} ring(s) ${rings.map((r) => r.length).join("/")} points, x ${Math.min(...all.map((p) => p[0])).toFixed(4)} ${Math.max(...all.map((p) => p[0])).toFixed(4)} y ${Math.min(...all.map((p) => p[1])).toFixed(4)} ${Math.max(...all.map((p) => p[1])).toFixed(4)}, placed off ${(off * 1000).toFixed(3)} mm`);
}
if (out !== "-") await writeOps(out!, mergeOps(await readOps(out!), ops));
else console.log(ops.map((o) => JSON.stringify(o)).join("\n"));
