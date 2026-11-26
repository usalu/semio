/** 🔎️ Which findings the `volumes-feature` rule gives for the goat's coarse horn when a station is thinned (closing diagnosis of `🧊️model-silhouette`). */
import { readFileSync } from "node:fs";
import { featureRule, thinnestDrawn } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
const S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const goat = JSON.parse(readFileSync(`${S}/🗂️catalog/🐕️animals/goat/🔣️.json`, "utf8")).asset;
const coarse = goat.volumes[500].body.find((s: { id: string }) => s.id === "horn-n");
const index = goat.volumes[500].body.indexOf(coarse);
const show = (label: string) => console.log(label, JSON.stringify(featureRule(goat, [500]).findings.map((f) => [f.path, f.message?.en?.slice(0, 120)])), "thinnest head", thinnestDrawn(goat, 500).get("head"));
console.log("index", index, "stations", coarse.sections.length);
coarse.id = "near-horn";
show("as is");
for (const factor of [0.5, 0.25, 0.1]) {
  const station = coarse.sections[1];
  const rx = station.rx;
  station.rx = rx * factor;
  show(`station1 rx x${factor}`);
  station.rx = rx;
}
