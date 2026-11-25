/** ⏱️ Which shapes of a scale cost the time of one evaluated state: `bun shape-time.ts <id|file.json> <scale> [pose…]` evaluates copies of the asset whose geometry at that scale is one single shape (after one evaluation that warms the caches) and prints the twelve slowest with their view, solid and number of points, and the time of the whole state. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const [arg, key, ...poses] = process.argv.slice(2);
const asset = readDoc(arg!).doc.asset;
const geometry = asset.representations[key!].geometry;
const only = (view: string, shape: any): any => ({ ...asset, representations: { ...asset.representations, [key!]: { ...asset.representations[key!], geometry: { ...geometry, elevation: [], front: [], plan: [], [view]: shape === null ? [] : [shape] } } } });
const selector = { kind: "scale", scale: { denominator: Number(key) } };
const time = (subject: any, pose: string): number => { const start = performance.now(); evaluate(subject, { kind: "pose", pose }, selector); return performance.now() - start; };
for (const pose of poses.length > 0 ? poses : [asset.presentation.defaultPose]) {
  time(only("front", null), pose);
  const rows = ["elevation", "front", "plan"].flatMap((view) => geometry[view].map((shape: any) => ({ view, id: shape.id, solid: shape.solid === undefined ? shape.kind : `${shape.solid.kind} ${shape.solid.sweep ?? ""}`, points: shape.points?.length ?? shape.d?.length ?? 0, ms: time(only(view, shape), pose) })));
  rows.sort((a: any, b: any) => b.ms - a.ms);
  console.log(`${asset.id} 1:${key} ${pose}: whole state ${time(asset, pose).toFixed(0)} ms, shapes ${rows.reduce((sum: number, row: any) => sum + row.ms, 0).toFixed(0)} ms`);
  for (const row of rows.slice(0, 12)) console.log(`  ${row.ms.toFixed(0)} ms ${row.view} ${row.id} ${row.solid} ${row.points}`);
}
