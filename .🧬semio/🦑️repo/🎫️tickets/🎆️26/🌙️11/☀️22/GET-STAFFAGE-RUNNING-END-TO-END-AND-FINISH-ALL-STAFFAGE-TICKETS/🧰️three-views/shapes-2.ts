/** 🔬️ Lists the extent of every evaluated shape of one asset at one scale and one pose or clip instant, per view, and the world frame of every bone, to find the shape that puts a view out of step with the other two.
 *
 * Usage: `bun shapes-2.ts <id> <scale> pose <pose> [--file <json>] [--match <text in shape or bone id>] [--bones]` or `bun shapes-2.ts <id> <scale> clip <clip> <t> […]`
 * @see ./sweep-2.ts */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { VIEWS, catalog, extentOf } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";

if (import.meta.main) {
  const args = process.argv.slice(2);
  const option = (name: string): string | undefined => (args.indexOf(name) < 0 ? undefined : args[args.indexOf(name) + 1]);
  const [id, scale, kind, name, time] = args;
  const file = option("--file");
  const match = option("--match");
  const asset = file === undefined ? catalog().find((candidate) => candidate.id === id)! : (JSON.parse(readFileSync(file, "utf8")) as { asset: ReturnType<typeof catalog>[number] }).asset;
  const query = kind === "pose" ? ({ kind: "pose", pose: name! } as const) : ({ kind: "clip", clip: name!, timeS: Number(time) } as const);
  const state = evaluate(asset, query, { kind: "scale", scale: { denominator: Number(scale) } });
  if (args.includes("--bones")) for (const [bone, frame] of Object.entries(state.bones)) if (match === undefined || bone.includes(match)) console.log(`bone ${bone}: ${JSON.stringify(frame, (_, v) => (typeof v === "number" ? Math.round(v * 1e4) / 1e4 : v))}`);
  for (const view of VIEWS)
    for (const evaluated of state.shapes[view]) {
      if (match !== undefined && !evaluated.shape.id.includes(match) && !evaluated.shape.bone.includes(match)) continue;
      const e = extentOf([evaluated], true);
      if (e === undefined) continue;
      console.log(`${view} ${evaluated.shape.id} (${evaluated.shape.bone}, ${evaluated.shape.role}, ${evaluated.shape.kind}${"solid" in evaluated.shape && evaluated.shape.solid !== undefined ? ", solid" : ""}): x [${e.minX.toFixed(3)}, ${e.maxX.toFixed(3)}] y [${e.minY.toFixed(3)}, ${e.maxY.toFixed(3)}]`);
    }
}
