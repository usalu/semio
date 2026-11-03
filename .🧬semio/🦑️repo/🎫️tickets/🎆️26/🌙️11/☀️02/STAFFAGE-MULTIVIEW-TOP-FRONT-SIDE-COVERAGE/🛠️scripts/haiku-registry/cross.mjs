// 🔗 Read-only cross-check: manifest representation status against each asset file's representations, reasons and viewpoint geometry keys.
import { readFileSync, readdirSync, statSync, existsSync } from "node:fs";
import { join } from "node:path";

const [manifestPath, catalogDir] = process.argv.slice(2);
const manifest = JSON.parse(readFileSync(manifestPath, "utf8"));
const folderOf = new Map(manifest.categories.map((c) => [c.id, c.folder]));
const jsonIn = (p) => readdirSync(p).filter((n) => n.endsWith(".json") && statSync(join(p, n)).isFile());
const mismatches = [];
const noReason = [];
const notInFile = [];
const noAssetFile = [];
const viewKeys = {};
let checked = 0;

for (const e of manifest.entries) {
  const dir = join(catalogDir, folderOf.get(e.category), e.id);
  if (!existsSync(dir)) { noAssetFile.push(e.id); continue; }
  const files = jsonIn(dir);
  if (files.length !== 1) { noAssetFile.push(`${e.id}(${files.length})`); continue; }
  const asset = JSON.parse(readFileSync(join(dir, files[0]), "utf8"));
  checked++;
  const reps = asset.representations ?? asset.representation ?? null;
  if (!reps) { notInFile.push(`${e.id}:no-representations-key`); continue; }
  const list = Array.isArray(reps) ? reps : Object.values(reps);
  for (const scale of ["50", "100", "200", "500"]) {
    const manifestStatus = e.representations[scale];
    const r = Array.isArray(reps) ? reps.find((x) => String(x.denominator ?? x.scale) === scale) : reps[scale];
    if (!r) { notInFile.push(`${e.id}@${scale}`); continue; }
    const fileStatus = r.status ?? "authored";
    if (manifestStatus === "unavailable" && fileStatus !== "unavailable") mismatches.push(`${e.id}@${scale}: manifest unavailable, file ${fileStatus}`);
    if (manifestStatus === "authored" && fileStatus === "unavailable") mismatches.push(`${e.id}@${scale}: manifest authored, file unavailable`);
    if (fileStatus === "unavailable" && !r.reason) noReason.push(`${e.id}@${scale}`);
    if (fileStatus !== "unavailable") {
      for (const v of ["elevation", "front", "plan"]) {
        const g = r.geometry ?? r;
        const key = g && g[v] !== undefined ? "has" : "missing";
        viewKeys[`${v}:${key}`] = (viewKeys[`${v}:${key}`] ?? 0) + 1;
      }
    }
  }
}

console.log(`[checkedAssetFiles] ${checked} of ${manifest.entries.length}`);
console.log(`[noAssetFile] ${noAssetFile.join(", ") || "none"}`);
console.log(`[statusMismatches] ${mismatches.length} ${mismatches.slice(0, 40).join(" | ") || "none"}`);
console.log(`[unavailableWithoutReason] ${noReason.length} ${noReason.slice(0, 40).join(", ") || "none"}`);
console.log(`[missingScaleOrKey] ${notInFile.length} ${notInFile.slice(0, 40).join(", ") || "none"}`);
console.log(`[viewKeysOnAuthoredScales] ${JSON.stringify(viewKeys)}`);
