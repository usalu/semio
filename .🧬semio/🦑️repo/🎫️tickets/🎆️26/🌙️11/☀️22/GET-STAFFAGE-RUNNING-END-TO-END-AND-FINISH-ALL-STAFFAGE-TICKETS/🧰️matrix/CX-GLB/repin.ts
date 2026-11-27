import { readFileSync, writeFileSync } from "node:fs";
import { writeGlb } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts";
import { glbDigestOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🧊️digests/🟦️.ts";
import { scaledFigure } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🧊️volumes/🟦️.ts";
const file = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🧊️model-digests/🔣️.json";
const text = readFileSync(file, "utf8");
const json = JSON.parse(text);
const asset = scaledFigure();
for (const pose of asset.poses.filter((p: any) => p.key !== true)) {
  const next = glbDigestOf(await writeGlb(asset, { pose: pose.id }));
  const old = json.glb[pose.id];
  const changed = Object.keys({ ...old, ...next }).filter((k) => JSON.stringify(old[k]) !== JSON.stringify((next as any)[k]));
  console.log("[DEBUG]", pose.id, "changed:", changed.join(","), old.byteLength, "->", (next as any).byteLength);
  json.glb[pose.id] = next;
}
writeFileSync(file, JSON.stringify(json, null, 1) + (text.endsWith("\n") ? "\n" : ""));
