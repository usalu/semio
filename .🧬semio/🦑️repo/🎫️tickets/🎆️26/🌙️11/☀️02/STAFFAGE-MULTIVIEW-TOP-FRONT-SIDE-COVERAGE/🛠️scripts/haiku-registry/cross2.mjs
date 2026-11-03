// 🔗 Read-only cross-check of manifest scale status against each asset file's representations and view geometry.
import { readFileSync, readdirSync, statSync, existsSync } from "node:fs";
import { join } from "node:path";

const [manifestPath, catalogDir] = process.argv.slice(2);
const manifest = JSON.parse(readFileSync(manifestPath, "utf8"));
const folderOf = new Map(manifest.categories.map((c) => [c.id, c.folder]));
const jsonIn = (p) => readdirSync(p).filter((n) => n.endsWith(".json") && statSync(join(p, n)).isFile());
const mismatches = [];
const noReason = [];
const missing = [];
const noAssetFile = [];
const views = {};
const geomShapes = {};
let checked = 0;

for (const e of manifest.entries) {
  const dir = join(catalogDir, folderOf.get(e.category), e.id);
  if (!existsSync(dir)) { noAssetFile.push(e.id); continue; }
  const files = jsonIn(dir);
  if (files.length !== 1) { noAssetFile.push(`${e.id}(${files.length})`); continue; }
  const asset = JSON.parse(readFileSync(join(dir, files[0]), "utf8")).asset;
  checked++;
  const reps = asset.representations;
  for (const scale of ["50", "100", "200", "500"]) {
    const manifestStatus = e.representations[scale];
    const r = reps?.[scale];
    if (!r) { missing.push(`${e.id}@${scale}`); continue; }
    const fileStatus = r.status;
    if (manifestStatus !== fileStatus) mismatches.push(`${e.id}@${scale}: manifest ${manifestStatus}, file ${fileStatus}`);
    if (fileStatus === "unavailable") {
      if (!r.reason) noReason.push(`${e.id}@${scale}`);
      continue;
    }
    for (const v of ["elevation", "front", "plan"]) {
      const k = r.geometry?.[v] !== undefined ? "present" : "absent";
      views[`${v}:${k}`] = (views[`${v}:${k}`] ?? 0) + 1;
    }
    const g = r.geometry ?? {};
    const shapeKey = Object.keys(g).sort().join(",");
    geomShapes[shapeKey] = (geomShapes[shapeKey] ?? 0) + 1;
  }
}

console.log(`[checkedAssetFiles] ${checked} of ${manifest.entries.length}`);
console.log(`[noAssetFile] ${noAssetFile.join(", ") || "none"}`);
console.log(`[statusMismatches] ${mismatches.length} ${mismatches.slice(0, 40).join(" | ") || "none"}`);
console.log(`[unavailableWithoutReason] ${noReason.length} ${noReason.join(", ") || "none"}`);
console.log(`[missingScaleInFile] ${missing.length} ${missing.slice(0, 40).join(", ") || "none"}`);
console.log(`[authoredScaleViews] ${JSON.stringify(views)}`);
console.log(`[authoredGeometryKeySets] ${JSON.stringify(geomShapes)}`);
