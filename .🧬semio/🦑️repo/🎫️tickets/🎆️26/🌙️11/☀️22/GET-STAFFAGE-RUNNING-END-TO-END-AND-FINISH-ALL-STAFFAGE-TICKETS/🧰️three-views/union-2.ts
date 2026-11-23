/** 🔺️ Prints the polygons that `unionPolygons` could not unite at one clip instant of one asset (the `input` of the `PolygonUnionError`), with full precision, as the measurement of a request against the polygon module.
 *
 * Usage: `bun union-2.ts <id> <scale> <clip> <t> [--file <json>]`
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔺️polygon/🟦️.ts */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { catalog } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";

if (import.meta.main) {
  const args = process.argv.slice(2);
  const [id, scale, clip, t] = args;
  const file = args.indexOf("--file") < 0 ? undefined : args[args.indexOf("--file") + 1];
  const asset = file === undefined ? catalog().find((candidate) => candidate.id === id)! : (JSON.parse(readFileSync(file, "utf8")) as { asset: ReturnType<typeof catalog>[number] }).asset;
  try {
    evaluate(asset, { kind: "clip", clip: clip!, timeS: Number(t) }, { kind: "scale", scale: { denominator: Number(scale) } });
    console.log("evaluates");
  } catch (error) {
    const input = (error as { input?: readonly (readonly { x: number; y: number }[])[] }).input ?? [];
    console.log((error as Error).message);
    console.log(JSON.stringify(input.map((ring) => ring.map((p) => [p.x, p.y]))));
  }
}
