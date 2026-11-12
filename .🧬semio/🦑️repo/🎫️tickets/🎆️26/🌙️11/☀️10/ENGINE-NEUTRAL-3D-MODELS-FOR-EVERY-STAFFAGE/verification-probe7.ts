import { readFileSync } from "node:fs";
const S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { writeGlb } = await import(`${S}/🔨️modules/📤️export/🧊️glb/🟦️.ts`);
const { loadGlb } = await import(`${S}/🧪️tests/🧰️support/🧊️.ts`);
const { visibleGlyphs } = await import(`${S}/🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts`);
const a = JSON.parse(readFileSync(`${S}/🗂️catalog/🧱️components/cantilever-balcony/🔣️.json`, "utf8")).asset;
const { parser } = await loadGlb(await writeGlb(a, { pose: a.poses[0].id, scale: 50 }));
for (const [v, e] of Object.entries<any>(visibleGlyphs(parser.json))) console.log(v, e.triangles.size, e.lines.size);
