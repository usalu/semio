import { readFileSync } from "node:fs";
import { measureVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes-next/🟦️.ts";
const a = JSON.parse(readFileSync("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/👤️sample-figure/🔣️.json", "utf8")).asset;
for (const r of [undefined, "far", "near"]) {
  const b = structuredClone(a);
  if (r !== undefined || true) b.representations["200"].sections = { front: { plane: Number(process.argv[2] ?? 0), label: { en: "A-A", de: "A-A" }, ...(r ? { retained: r } : {}) } };
  console.log(r, measureVolumes(b, { scale: 200, rules: ["volumes-silhouette"] }).filter((m) => m.name.endsWith(":front")).map((m) => `${m.name}=${m.value.toFixed(4)}`).join(" "));
}
