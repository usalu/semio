/** 🔬️ Compares two asset documents anchor by anchor (by id): the largest move of every anchor per pose, with the worst anchor named. Placed 2D shapes carry no id and change their order with depth, so they are not compared here.
 * Usage: `bun rework-goat-compare.ts <before asset document json> <after asset document json> [scale]` */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const [a, b] = process.argv.slice(2, 4).map((path) => JSON.parse(readFileSync(path!, "utf8")).asset);
const scale = Number(process.argv[4] ?? 50);
for (const pose of a.poses) {
  const query = { kind: "pose", pose: pose.id };
  const [x, y] = [evaluate(a, query, { kind: "scale", scale: { denominator: scale } }), evaluate(b, query, { kind: "scale", scale: { denominator: scale } })];
  let [worst, name] = [0, ""];
  for (const id of Object.keys(x.anchors)) {
    const move = Math.hypot(x.anchors[id]!.at.x - y.anchors[id]!.at.x, x.anchors[id]!.at.y - y.anchors[id]!.at.y, x.anchors[id]!.z - y.anchors[id]!.z);
    if (move > worst) [worst, name] = [move, id];
  }
  process.stdout.write(`1:${scale} ${pose.id}: anchor ${(worst * 1000).toFixed(2)} mm (${name})\n`);
}
