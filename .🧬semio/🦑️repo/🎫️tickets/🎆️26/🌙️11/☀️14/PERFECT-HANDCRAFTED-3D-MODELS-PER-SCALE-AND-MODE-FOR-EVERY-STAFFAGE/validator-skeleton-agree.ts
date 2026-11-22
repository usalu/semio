/** 🦴️ Whether `volumes-feature` and `volumes-slender` accept the structure shafts that `🧪️tests/🧊️model-skeleton` pins at their true section at every scale. Usage: `bun validator-skeleton-agree.ts` */
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { featureRule, slenderRule, sortedExtents } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
const S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const f = JSON.parse(readFileSync(join(S, "🧫️fixtures/🧊️model-skeleton/🔣️.json"), "utf8")).adultLongBones;
console.log("shafts", JSON.stringify(f.pairedShafts), "subjects", f.subjects.length);
const humans = readdirSync(join(S, "🗂️catalog")).find((n) => n.includes("humans"))!;
for (const name of f.subjects) {
  const asset = JSON.parse(readFileSync(join(S, "🗂️catalog", humans, name, "🔣️.json"), "utf8")).asset;
  const ids = new Set(f.pairedShafts.flatMap((s: string) => [`${s}-l`, `${s}-r`]));
  const bad = [...featureRule(asset).findings, ...slenderRule(asset).findings].filter((x) => x.list === "structure" && [...ids].some((id) => x.message.en.includes(`"${id}"`)));
  const v = asset.volumes["500"];
  const shaft = v?.structure?.find((s: { id: string }) => ids.has(s.id));
  console.log(name, bad.length, bad.slice(0, 2).map((x) => `1:${x.scale} ${x.rule} ${x.message.en.slice(0, 110)}`).join(" | "), shaft ? `500:${shaft.id} slender=${shaft.slender} ext=${sortedExtents(shaft).map((e) => (e * 1000).toFixed(0)).join("x")}` : "");
}
