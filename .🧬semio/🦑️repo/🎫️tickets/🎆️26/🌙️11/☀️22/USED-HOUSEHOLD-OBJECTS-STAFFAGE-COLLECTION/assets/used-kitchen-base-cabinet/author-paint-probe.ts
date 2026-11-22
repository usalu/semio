/** 🎨 Paint probe of the candidate: prints the prepared shapes of one view of one pose at one scale with their paint order and paper boxes.
 *
 * `$BUN author-paint-probe.ts <scale> <pose> <view> [denominator]`
 *
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts — `prepare`
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { prepare, sourceOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts";
import { evaluateDrawing } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const asset = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")).asset;
const [scale, pose, view, at] = process.argv.slice(2);
const state = evaluateDrawing(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: Number(scale) } });
const k = 1000 / Number(at ?? scale);
for (const p of prepare(state, view as never, sourceOf(state), k)) console.log(`[DEBUG] ${p.order} ${p.shape.id} z${p.shape.z} pen ${p.weightMm} box x ${p.box.minX.toFixed(3)}..${p.box.maxX.toFixed(3)} y ${p.box.minY.toFixed(3)}..${p.box.maxY.toFixed(3)}`);
