/** 🧵️ Redraws the open stroke of a slender solid along the centre line the guard compares it by (`strokesOf`: the centre line taken from the mesh, which rounds the bends of a bent loft and runs to the cap tips), so that the drawn tube and the compared tube are one line; writes `shape-set` ops (points only) for shapes that carry the id of their solid, optional base ops applied first.
 *  bun centre.ts <slug|file.json> <scale> <view> <ops-out.json> <shape id>… [--base ops.json] */
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { drawnFor, strokeDrawings, strokesOf, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { applyOps, mergeOps, readOps, writeOps, type Op } from "./ops.ts";
import { fileOf } from "./people.ts";

const args = process.argv.slice(2);
const baseAt = args.indexOf("--base");
const base = baseAt < 0 ? undefined : args.splice(baseAt, 2)[1];
const [name, scaleArg, view, out, ...ids] = args as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
if (base !== undefined) applyOps(asset, await readOps(base));
const scale = Number(scaleArg), tolerance = featureOf(scale) / 40;
const sel = { kind: "scale", scale: { denominator: scale } } as const;
const pose = { kind: "pose", pose: asset.presentation.defaultPose } as const;
const state: any = evaluate(asset, pose, sel);
const solids = (worldSolids(asset, state.bones as never, scale) as any[]).filter((w) => w.texture !== true);
const drawn = strokeDrawings(view as never, solids, drawnFor(asset, state, view as never, scale));
const r5 = (n: number): number => Math.round(n * 1e5) / 1e5;
type Q = { x: number; y: number };
const thin = (line: Q[]): Q[] => {
  if (line.length < 3) return line;
  const [a, b] = [line[0]!, line[line.length - 1]!];
  const l = Math.hypot(b.x - a.x, b.y - a.y) || 1;
  let [worst, at] = [0, 0];
  for (let i = 1; i < line.length - 1; i++) { const d = Math.abs((b.x - a.x) * (line[i]!.y - a.y) - (b.y - a.y) * (line[i]!.x - a.x)) / l; if (d > worst) [worst, at] = [d, i]; }
  return worst <= tolerance ? [a, b] : [...thin(line.slice(0, at + 1)).slice(0, -1), ...thin(line.slice(at))];
};
const ops: Op[] = [];
for (const id of ids) {
  const solid = solids.find((w) => w.id === id && w.slender === true);
  const shape = (asset.representations[scaleArg!].geometry[view!] as any[]).find((s) => s.id === id);
  if (solid === undefined || shape === undefined || shape.closed !== false || shape.solid !== undefined) throw new Error(`${id}: needs a slender solid and a flat open stroke of that id`);
  const quads = strokesOf(view as never, solid, drawn) as Q[][];
  const mid = (a: Q, b: Q): Q => ({ x: (a.x + b.x) / 2, y: (a.y + b.y) / 2 });
  const ends = quads.map((q) => { const short = Math.hypot(q[0]!.x - q[1]!.x, q[0]!.y - q[1]!.y) <= Math.hypot(q[1]!.x - q[2]!.x, q[1]!.y - q[2]!.y); return short ? [mid(q[0]!, q[1]!), mid(q[2]!, q[3]!)] : [mid(q[1]!, q[2]!), mid(q[3]!, q[0]!)]; });
  const line: Q[] = [];
  ends.forEach(([a, b], i) => { const [p, q] = i === 0 ? (ends.length > 1 && Math.hypot(a!.x - ends[1]![0]!.x, a!.y - ends[1]![0]!.y) + Math.hypot(a!.x - ends[1]![1]!.x, a!.y - ends[1]![1]!.y) < Math.hypot(b!.x - ends[1]![0]!.x, b!.y - ends[1]![0]!.y) + Math.hypot(b!.x - ends[1]![1]!.x, b!.y - ends[1]![1]!.y) ? [b!, a!] : [a!, b!]) : Math.hypot(a!.x - line[line.length - 1]!.x, a!.y - line[line.length - 1]!.y) <= Math.hypot(b!.x - line[line.length - 1]!.x, b!.y - line[line.length - 1]!.y) ? [a!, b!] : [b!, a!]; if (i === 0) line.push(p); line.push(q); });
  const copy = structuredClone(asset);
  copy.representations[scaleArg!].geometry[view!].push({ id: "probe-centre", bone: shape.bone, role: "body", z: 0, filled: true, tone: 0.5, kind: "poly", points: [[0, 0], [1, 0], [0, 1]], closed: true });
  const [o, x, y] = (evaluate(copy, pose, sel) as any).shapes[view!].find((s: any) => s.shape.id === "probe-centre").rings[0] as Q[];
  const [a, b, c, d] = [x!.x - o!.x, y!.x - o!.x, x!.y - o!.y, y!.y - o!.y];
  const det = a * d - b * c;
  const points = thin(line).map((p) => [r5((d * (p.x - o!.x) - b * (p.y - o!.y)) / det), r5((-c * (p.x - o!.x) + a * (p.y - o!.y)) / det)]);
  ops.push({ op: "shape-set", scale: scaleArg, view, id, set: { points } });
  console.log(`${id}: ${quads.length} compared segments, ${points.length} points`);
}
if (out !== "-") await writeOps(out!, mergeOps(await readOps(out!), ops)); else console.log(ops.map((o) => JSON.stringify(o)).join("\n"));
