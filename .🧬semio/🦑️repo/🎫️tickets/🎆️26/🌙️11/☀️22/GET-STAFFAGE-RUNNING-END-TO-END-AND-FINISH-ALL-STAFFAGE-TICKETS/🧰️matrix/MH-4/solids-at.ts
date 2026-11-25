/** 🧲️ Which body solids and which drawn shapes lie at a place of a view in the default pose: `bun solids-at.ts <id|file.json> --scale 100 --view plan --at 0.104,0.197 [--reach 0.02]` prints every body solid whose projected box comes within `reach` of the point (view metres, as `where.ts` prints them) with its bone and box, then every drawn shape of the view likewise. Run it from `S/📦️packages/🟦️typescript`. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";
const { worldSolids, project } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const { evaluate } = await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"));
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const asset = readDoc(args[0]!).doc.asset;
const scale = Number(flag("scale"));
const view = flag("view")!;
const [px, py] = flag("at")!.split(",").map(Number) as [number, number];
const reach = Number(flag("reach") ?? 0.02);
const state = evaluate(asset, { kind: "pose", pose: flag("pose") ?? asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const near = (x0: number, x1: number, y0: number, y1: number): boolean => px > x0 - reach && px < x1 + reach && py > y0 - reach && py < y1 + reach;
for (const world of worldSolids(asset, state.bones, scale) as any[]) {
  let [x0, x1, y0, y1] = [Infinity, -Infinity, Infinity, -Infinity];
  for (let i = 0; i < world.positions.length; i += 3) {
    const p = project(view, world.positions[i], world.positions[i + 1], world.positions[i + 2]);
    [x0, x1, y0, y1] = [Math.min(x0, p.x), Math.max(x1, p.x), Math.min(y0, p.y), Math.max(y1, p.y)];
  }
  if (near(x0, x1, y0, y1)) console.log(`BODY ${world.id} on ${world.bone}: x [${x0.toFixed(3)}, ${x1.toFixed(3)}] y [${y0.toFixed(3)}, ${y1.toFixed(3)}]`);
}
for (const shape of state.shapes[view] as any[]) {
  const points = shape.rings.flat();
  if (points.length === 0) continue;
  const [x0, x1, y0, y1] = [Math.min(...points.map((p: any) => p.x)), Math.max(...points.map((p: any) => p.x)), Math.min(...points.map((p: any) => p.y)), Math.max(...points.map((p: any) => p.y))];
  if (near(x0, x1, y0, y1)) console.log(`DRAWN ${shape.shape.id} on ${shape.shape.bone} (${shape.shape.kind}${shape.shape.solid ? "/" + shape.shape.solid.kind : ""}, z ${shape.shape.z}): x [${x0.toFixed(3)}, ${x1.toFixed(3)}] y [${y0.toFixed(3)}, ${y1.toFixed(3)}]`);
}
