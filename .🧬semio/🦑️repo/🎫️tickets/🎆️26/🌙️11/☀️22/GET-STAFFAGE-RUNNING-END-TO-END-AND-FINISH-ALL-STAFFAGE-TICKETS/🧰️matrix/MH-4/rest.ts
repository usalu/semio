/** 🛌️ What a repair did to the drawing where nothing turns: `bun rest.ts <before.json> <after.json> [--pose <id>] [--limit mm]` evaluates both documents at every authored scale in a pose without any bone offset (a probe pose added to a copy in memory; `--pose` takes a pose of the asset instead) and prints every shape whose bounding box in a view moved by more than the limit (default 1 mm), the shapes whose place in the painting order changed, and the extents of the three views. A solid that keeps the outline of the flat shape it replaces shows nothing here. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const limit = Number(flag("limit") ?? 1) / 1000;
const VIEWS = ["elevation", "front", "plan"] as const;
const load = (arg: string): any => {
  const asset = structuredClone(readDoc(arg).doc.asset);
  if (flag("pose") === undefined) asset.poses.push({ ...structuredClone(asset.poses.find((pose: any) => pose.id === asset.presentation.defaultPose)), id: "rest-probe", bones: {} });
  return asset;
};
const [before, after] = [load(args[0]!), load(args[1]!)];
const pose = flag("pose") ?? "rest-probe";
const box = (rings: any[]): number[] | undefined => {
  const points = rings.flat();
  return points.length === 0 ? undefined : [Math.min(...points.map((p: any) => p.x)), Math.max(...points.map((p: any) => p.x)), Math.min(...points.map((p: any) => p.y)), Math.max(...points.map((p: any) => p.y))];
};
let moved = 0;
for (const key of Object.keys(after.representations).filter((k) => engine.authoredOf(after, k) !== undefined)) {
  const [a, b] = [before, after].map((asset) => evaluate(asset, { kind: "pose", pose }, engine.scaleSelector(Number(key))));
  for (const view of VIEWS) {
    const old = new Map<string, any>(a.shapes[view].map((shape: any) => [`${shape.shape.bone}/${shape.shape.id}`, shape]));
    for (const shape of b.shapes[view]) {
      const name = `${shape.shape.bone}/${shape.shape.id}`;
      const [x, y] = [box(old.get(name)?.rings ?? []), box(shape.rings)];
      if (x === undefined || y === undefined) {
        if ((x === undefined) !== (y === undefined)) console.log(`1:${key} ${view} ${name}: ${x === undefined ? "new or empty before" : "empty now"}`);
        continue;
      }
      const delta = Math.max(...x.map((value, i) => Math.abs(value - y[i]!)));
      if (delta > limit) (moved++, console.log(`1:${key} ${view} ${name}: box moved ${(delta * 1000).toFixed(1)} mm  [${x.map((value) => value.toFixed(3))}] -> [${y.map((value) => value.toFixed(3))}]`));
    }
    const order = (state: any): string[] => state.shapes[view].map((shape: any) => `${shape.shape.bone}/${shape.shape.id}`);
    const [oa, ob] = [order(a), order(b)];
    const shifted = ob.filter((name, i) => oa[i] !== name);
    if (shifted.length > 0) console.log(`1:${key} ${view} painting order differs for ${shifted.length} shapes: ${shifted.slice(0, 12).join(" ")}`);
    const extent = (state: any): string => (box(state.shapes[view].filter((shape: any) => shape.shape.role !== "ground").flatMap((shape: any) => shape.rings)) ?? []).map((value) => value.toFixed(3)).join(", ");
    if (extent(a) !== extent(b)) console.log(`1:${key} ${view} extent [${extent(a)}] -> [${extent(b)}]`);
  }
}
console.log(`${after.id} ${pose}: ${moved} shapes moved by more than ${(limit * 1000).toFixed(1)} mm`);
