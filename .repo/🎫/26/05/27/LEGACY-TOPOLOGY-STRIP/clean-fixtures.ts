/** @emoji 🧹 Remove __removed_* keys from spatial fixtures after cluster/cellComplex strip. */
import { readFileSync, writeFileSync } from "node:fs";

const files = [
	"c:/git/compose/spatial/fixtures/simple.spatial.json",
	"c:/git/compose/spatial/fixtures/spatial.spatial.json",
	"c:/git/compose/spatial/fixtures/small-building.model.json",
	"c:/git/compose/spatial/fixtures/tall-building.model.json",
	"c:/git/compose/spatial/fixtures/large-building.model.json",
	"c:/git/compose/spatial/fixtures/geometry.json",
	"c:/git/compose/spatial/fixtures/geometry-loom.json",
	"c:/git/compose/spatial/fixtures/geometry-routes.json",
];

for (const f of files) {
	let j = readFileSync(f, "utf8");
	j = j.replace(/\s*,?\s*"__removed_cellComplexes":\s*\[[^\]]*\]/gs, "");
	j = j.replace(/\s*,?\s*"__removed_clusters":\s*\[[\s\S]*?\]/g, "");
	j = j.replace(/,\s*}/g, " }").replace(/,\s*]/g, " ]");
	writeFileSync(f, j);
}

console.log("clean-fixtures done");
