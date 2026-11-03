import { readFileSync, writeFileSync } from "node:fs";

const S = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
const a = JSON.parse(readFileSync(`${S}/🏗️machinery/solar-thermal-collector/🔣️.json`, "utf8")).asset;
const row = {
  id: a.id,
  category: a.category,
  title: a.title,
  heightM: a.dimensions.heightM,
  widthM: a.dimensions.widthM,
  poses: a.poses.filter((p) => !p.key).map((p) => p.id),
  clips: a.clips.map((c) => c.id),
  representations: Object.fromEntries(["50", "100", "200", "500"].map((k) => [k, a.representations[k].status])),
  symbols: [],
  analytics: ["thermal", ...(a.thermal.emitters.length ? ["transport"] : []), "structural"],
  viewpoints: a.presentation.viewpoints,
  exports: a.exports,
  distinction: {
    en: "A 2.0 m by 1.0 m glazed flat-plate solar collector tilted by 35 degrees on a steel stand, whose absorber heats to stagnation in the sun and cools while the pump runs.",
    de: "Ein verglaster Flachkollektor von 2,0 m mal 1,0 m, 35 Grad geneigt auf einem Stahlgestell, dessen Absorber in der Sonne bis zum Stillstand heiß wird und bei laufender Pumpe abkühlt.",
  },
  completion: "authored",
};
writeFileSync(process.argv[2], JSON.stringify(row, null, 2).split("\n").map((l) => "    " + l).join("\n") + ",\n");
