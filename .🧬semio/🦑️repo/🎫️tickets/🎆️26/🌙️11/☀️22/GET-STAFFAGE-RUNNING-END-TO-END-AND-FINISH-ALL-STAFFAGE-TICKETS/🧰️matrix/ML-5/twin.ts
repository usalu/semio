/** 👯️ Each drawn shape beside the body solid of its id in the default pose: `bun twin.ts <id|file.json> --scale 50 [--view elevation] [--bones <regex>] [--min 5] [--pose <id>]` prints, per view, the box of the drawn rings and the box of the projected solid (view metres) for every shape whose box leaves the box of its solid by at least `min` millimetres on a side, and the solids of the bones that no shape of that id draws. Run it from `T`.
 * @see ../MH-4/solids-at.ts — the projection of the solids */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { worldSolids, project } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const { evaluate } = await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"));
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const asset = readDoc(args[0]!).doc.asset;
const scale = Number(flag("scale"));
const bones = new RegExp(flag("bones") ?? ".");
const min = Number(flag("min") ?? 5) / 1000;
const state = evaluate(asset, { kind: "pose", pose: flag("pose") ?? asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const worlds = (worldSolids(asset, state.bones, scale) as any[]).filter((world) => bones.test(world.bone ?? ""));
const text = (box: number[]): string => `x[${box[0]!.toFixed(3)},${box[1]!.toFixed(3)}] y[${box[2]!.toFixed(3)},${box[3]!.toFixed(3)}]`;
for (const view of flag("view") === undefined ? ["elevation", "front", "plan"] : [flag("view")!]) {
  console.log(`== ${asset.id} 1:${scale} ${view}`);
  const drawn = new Set<string>();
  for (const shape of state.shapes[view].filter((candidate: any) => bones.test(candidate.shape.bone ?? "") && candidate.rings.flat().length > 0)) {
    const points = shape.rings.flat();
    const a = [Math.min(...points.map((p: any) => p.x)), Math.max(...points.map((p: any) => p.x)), Math.min(...points.map((p: any) => p.y)), Math.max(...points.map((p: any) => p.y))];
    const world = worlds.find((candidate) => candidate.id === shape.shape.id);
    drawn.add(shape.shape.id);
    if (world === undefined) {
      console.log(`   ${shape.shape.bone}/${shape.shape.id}: drawn ${text(a)}, no solid of this id`);
      continue;
    }
    const b = [Infinity, -Infinity, Infinity, -Infinity];
    for (let i = 0; i < world.positions.length; i += 3) {
      const p = project(view, world.positions[i], world.positions[i + 1], world.positions[i + 2]);
      [b[0], b[1], b[2], b[3]] = [Math.min(b[0]!, p.x), Math.max(b[1]!, p.x), Math.min(b[2]!, p.y), Math.max(b[3]!, p.y)];
    }
    const off = Math.max(...a.map((value, k) => Math.abs(value - b[k]!)));
    if (off >= min) console.log(`   ${shape.shape.bone}/${shape.shape.id}: drawn ${text(a)} solid ${text(b)} off ${(off * 1000).toFixed(0)} mm`);
  }
  for (const world of worlds) if (!drawn.has(world.id)) console.log(`   ${world.bone}/${world.id}: solid without a shape of its id`);
}
