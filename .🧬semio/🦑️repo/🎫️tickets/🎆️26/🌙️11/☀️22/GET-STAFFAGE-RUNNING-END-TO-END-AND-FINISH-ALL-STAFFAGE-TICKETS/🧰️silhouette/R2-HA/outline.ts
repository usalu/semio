/** ✍️ Redraws parts of one view from the volume: per spec the outline of the projected body solids (their exact union, simplified to a fraction of the smallest feature) as a filled shape in the coordinates of its bone, written as `shape-add` ops (an existing shape of the id is replaced in place). The mapping from the view to the shape coordinates is probed through `evaluate` and the placed result is checked against the outline.
 *  bun outline.ts <id|file.json> <scale> <view> <ops-out.json|-> <spec>… [--base ops.json]
 *  spec: <shape id>=<solid id>[+<solid id>…][@key=value,…]; a solid id may end in `*`; keys: solid=half (the shape becomes the profile of a body of revolution about its bone without mirror: one outline, exact in the side or front view and carried in three dimensions), shift (dx:dy in mm: a flat polygon or stroke is moved by that much on the paper of the view), clip (mm: the shape is a flat open stroke and is cut back to where it lies inside the outline of the named solids, that far from its edge; outside altogether it is removed), solid (keep: the shape stays the profile of its body of revolution without mirror, the outline is written in the coordinates of that profile), after, like (shape whose properties are copied), bone, role, tone, z, region, material, pen, hull (1: convex hull), tol (mm), minus (solid ids joined by `+` whose outline is cut out), axis (1: an open stroke along the long axis of the projected solid at a pen of its width, for a slender bar; a bar seen end-on stays an outline), rm (1: write a shape-remove of the id instead) */
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
  for (const digits of [1e6, 1e5, 2e4, 1e4]) {
    try {
      const snap = (p: P): P => [Math.round(p[0] * digits) / digits, Math.round(p[1] * digits) / digits];
      const clean = tris.map((poly) => poly.map((ring) => ring.map(snap))).filter((poly) => { const [a, b, c] = poly[0] as [P, P, P]; return Math.abs((b[0] - a[0]) * (c[1] - a[1]) - (c[0] - a[0]) * (b[1] - a[1])) > 1e-10; });
      let acc: P[][][] = [];
      for (let i = 0; i < clean.length; i += 200) acc = polygonClipping.union(acc as never, ...(clean.slice(i, i + 200) as never[])) as never;
      return acc;
    } catch (error) { if (digits === 1e4) throw error; }
  }
  return [];
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
  const copy = structuredClone(asset);
  copy.representations[scaleArg!].geometry[view!].push({ id: "probe-outline", bone, role: "body", z: 0, filled: true, tone: 0.5, kind: "poly", points: [[0, 0], [1, 0], [0, 1]], closed: true, ...(solid === undefined ? {} : { solid }) });
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
  if (opt.shift !== undefined) {
    const flat = shapes.find((s) => s.id === id);
    if (flat === undefined || flat.kind !== "poly" || flat.solid !== undefined) throw new Error(`${id}: not a flat polygon or stroke`);
    const [dx, dy] = opt.shift.split(":").map((n) => Number(n) / 1000) as [number, number];
    const placedFlat = (evaluate(structuredClone(asset), pose, sel) as any).shapes[view!].find((x: any) => x.shape.id === id).rings[0] as { x: number; y: number }[];
    const back = probe(flat.bone);
    ops.push({ op: "shape-set", scale: scaleArg, view, id, set: { points: placedFlat.map((p) => back([p.x + dx, p.y + dy])) } });
    console.log(`${id}: moved by ${opt.shift} mm`);
    continue;
  }
  if (opt.clip !== undefined) {
    const stroke = shapes.find((s) => s.id === id);
    if (stroke === undefined || stroke.kind !== "poly" || stroke.closed !== false || stroke.solid !== undefined) throw new Error(`${id}: not a flat open stroke`);
    const body = union(pick(ids!));
    const inside = (p: P): boolean => { let c = false; for (const poly of body) for (const ring of poly) for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) if ((ring[i]![1] > p[1]) !== (ring[j]![1] > p[1]) && p[0] < ((ring[j]![0] - ring[i]![0]) * (p[1] - ring[i]![1])) / (ring[j]![1] - ring[i]![1]) + ring[i]![0]) c = !c; return c; };
    const edge = (p: P): number => { let m = Infinity; for (const poly of body) for (const ring of poly) for (let i = 0; i < ring.length - 1; i++) { const [a, b] = [ring[i]!, ring[i + 1]!]; const l = (b[0] - a[0]) ** 2 + (b[1] - a[1]) ** 2; const t = l === 0 ? 0 : Math.max(0, Math.min(1, ((p[0] - a[0]) * (b[0] - a[0]) + (p[1] - a[1]) * (b[1] - a[1])) / l)); m = Math.min(m, Math.hypot(p[0] - a[0] - t * (b[0] - a[0]), p[1] - a[1] - t * (b[1] - a[1]))); } return m; };
    const copy = structuredClone(asset);
    const placedStroke = ((evaluate(copy, pose, sel) as any).shapes[view!].find((x: any) => x.shape.id === id).rings[0] as { x: number; y: number }[]).map((p) => [p.x, p.y] as P);
    const back = probe(stroke.bone);
    const margin = Number(opt.clip) / 1000;
    const dense: P[] = [];
    for (let i = 0; i < placedStroke.length - 1; i++) for (let t = 0; t < 24; t++) dense.push([placedStroke[i]![0] + ((placedStroke[i + 1]![0] - placedStroke[i]![0]) * t) / 24, placedStroke[i]![1] + ((placedStroke[i + 1]![1] - placedStroke[i]![1]) * t) / 24]);
    dense.push(placedStroke[placedStroke.length - 1]!);
    let [best, run]: [P[], P[]] = [[], []];
    for (const p of dense) { if (inside(p) && edge(p) >= margin) run.push(p); else { if (run.length > best.length) best = run; run = []; } }
    if (run.length > best.length) best = run;
    if (best.length < 2) { ops.push({ op: "shape-remove", scale: scaleArg, view, id }); console.log(`${id}: lies outside the body, removed`); continue; }
    const kept = simplify([...best, best[0]!], 0.0003).map(back);
    const points = best.length === dense.length ? undefined : [best[0]!, ...best.slice(1, -1).filter((_, i) => i % 6 === 5), best[best.length - 1]!].map(back);
    void kept;
    if (points === undefined) { console.log(`${id}: lies inside the body`); continue; }
    ops.push({ op: "shape-set", scale: scaleArg, view, id, set: { points } });
    console.log(`${id}: clipped to the body, ${points.length} points of ${dense.length} samples kept ${best.length}`);
    continue;
  }
  const list = pick(ids!);
  const like = shapes.find((s) => s.id === (opt.like ?? id));
  const first = list[0];
  const bone = opt.bone ?? like?.bone ?? first.bone;
  const tol = (opt.tol === undefined ? (feature * 1000) / 60 : Number(opt.tol)) / 1000;
  let polys = union(list);
  if (opt.minus !== undefined) polys = polygonClipping.difference(polys as never, union(pick(opt.minus)) as never) as never;
  let rings: P[][] = opt.hull === "1" ? [hullOf(polys.flatMap((poly) => poly[0]!))] : polys.flatMap((poly) => poly.map((ring) => simplify(ring, tol)));
  rings = rings.filter((r) => r.length >= 3 && areaOf(r) > (feature / 20) ** 2);
  const kept = opt.solid === "keep" ? like?.solid : opt.solid === "half" ? { kind: "revolved", axisBone: opt.bone ?? like?.bone ?? first.bone, axis: "u", axisX: 0, axisZ: 0 } : undefined;
  if (opt.solid === "half" && view === "plan") throw new Error(`${id}: the plan draws a body of revolution by its circles, not by a half profile`);
  if (opt.solid === "keep" && (kept?.kind !== "revolved" || kept.mirror === true || view === "plan")) throw new Error(`${id}: only the half profile of a body of revolution without mirror keeps its solid in a side or front view`);
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
  const local: P[][] = rings.map((r) => r.map(toLocal));
  const region = opt.region ?? like?.region ?? (asset.volumes[scaleArg!].thermal as any[]).find((t) => t.of === first.id)?.region;
  const source = (asset.volumes[scaleArg!].body as any[]).find((s) => s.id === first.id);
  const material = opt.material ?? like?.material ?? source.material;
  const shape: any = { id, bone, role: opt.role ?? like?.role ?? (source.role === "detail" ? "detail" : "body"), z: Number(opt.z ?? like?.z ?? 3), filled: true, tone: Number(opt.tone ?? like?.tone ?? source.tone ?? 0.5) };
  if (opt.pen !== undefined) shape.penMm = Number(opt.pen); else if (like?.penMm !== undefined) shape.penMm = like.penMm;
  if (like?.hatch !== undefined && opt.hatch !== "0") shape.hatch = like.hatch;
  if (like?.regions !== undefined && opt.region === undefined) shape.regions = like.regions; else if (region !== undefined) shape.region = region;
  if (material !== undefined) shape.material = material;
  if (kept !== undefined) shape.solid = kept;
  if (stroke !== undefined) Object.assign(shape, { filled: false, penMm: Number(opt.pen ?? stroke.pen), kind: "poly", points: local[0], closed: false });
  else if (local.length === 1) Object.assign(shape, { kind: "poly", points: local[0], closed: true });
  else Object.assign(shape, { kind: "path", d: local.map((r) => `M ${r.map((p) => `${p[0]} ${p[1]}`).join(" L ")} Z`).join(" ") });
  const after = opt.after ?? (like === undefined ? shapes[shapes.length - 1]?.id : undefined);
  ops.push(like !== undefined && opt.like === undefined ? { op: "shape-add", scale: scaleArg, view, after: id, shape } : { op: "shape-add", scale: scaleArg, view, ...(after === undefined ? {} : { after }), shape });
  let off = Infinity;
  for (let round = 0; round < 6 && off > 2e-5 && stroke === undefined; round++) {
    const copy = structuredClone(asset);
    applyOps(copy, [ops[ops.length - 1]!]);
    const placed = (evaluate(copy, pose, sel) as any).shapes[view!].find((s: any) => s.shape.id === id).rings as { x: number; y: number }[][];
    off = 0;
    const origin = toLocal([0, 0]);
    const fixed = local.map((ring, i) => ring.map((l, j) => {
      const [p, q] = [placed[i]?.[j], rings[i]![j]!];
      if (p === undefined || placed[i]!.length !== ring.length) return l;
      off = Math.max(off, Math.hypot(p.x - q[0], p.y - q[1]));
      const d = toLocal([q[0] - p.x, q[1] - p.y]);
      return [r5(l[0] + d[0] - origin[0]), r5(l[1] + d[1] - origin[1])] as P;
    }));
    if (off <= 2e-5 || round === 5) break;
    if (shape.kind === "poly") shape.points = fixed[0];
    else shape.d = fixed.map((r) => `M ${r.map((p) => `${p[0]} ${p[1]}`).join(" L ")} Z`).join(" ");
    fixed.forEach((ring, i) => { local[i] = ring; });
  }
  const all = rings.flat();
  console.log(`${id}: ${list.map((w) => w.id).join("+")} on ${bone}, ${rings.length} ring(s) ${rings.map((r) => r.length).join("/")} points, x ${Math.min(...all.map((p) => p[0])).toFixed(4)} ${Math.max(...all.map((p) => p[0])).toFixed(4)} y ${Math.min(...all.map((p) => p[1])).toFixed(4)} ${Math.max(...all.map((p) => p[1])).toFixed(4)}, placed off ${(off * 1000).toFixed(3)} mm`);
}
if (out !== "-") await writeOps(out!, mergeOps(await readOps(out!), ops));
else console.log(ops.map((o) => JSON.stringify(o)).join("\n"));
