import { readFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { runPreview } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";

const ROOT = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances";
const OUT = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🗑️generated/P1/prev";
const [scalesArg, ...assets] = process.argv.slice(2);
const scales = scalesArg!.split(",");
for (const id of assets) {
  const doc = JSON.parse(readFileSync(join(ROOT, id, "🔣️.json"), "utf8")).asset;
  const dir = join(OUT, id);
  mkdirSync(dir, { recursive: true });
  for (const scale of scales) {
    for (const pose of doc.poses) await runPreview(["--asset", id, "--scale", scale, "--pose", pose.id, "--views", "--no-png", "--out", dir]);
    for (const clip of doc.clips) for (const f of [0, 0.5, 0.999]) await runPreview(["--asset", id, "--scale", scale, "--clip", clip.id, "--time", String(clip.durationS * f), "--views", "--no-png", "--out", dir]);
  }
}
console.log("done");
