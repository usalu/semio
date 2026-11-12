import { appendFileSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { assetPathsFromFiles, readStreamedAsset } = await import(`${S}/🔨️modules/🗂️catalog/📥️files/🟦️.ts`);
const { plannedNames } = await import(`${S}/🔨️modules/📤️export/📦️batch/🟦️.ts`);
const { auditFiles, auditGeometry } = await import(`${S}/🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts`);
const [out, log, only] = process.argv.slice(2);
const manifest = JSON.parse(readFileSync(join(out!, "manifest.json"), "utf8"));
writeFileSync(log!, "");
const t0 = Date.now();
for (const entry of await assetPathsFromFiles()) {
  if (only && !only.split(",").includes(entry.folder)) continue;
  const { asset, categoryFolder } = await readStreamedAsset(entry);
  const t = Date.now();
  const geometry = await auditGeometry(asset);
  const files = await auditFiles(asset, manifest.files, (f: string) => new Uint8Array(readFileSync(join(out!, f))), plannedNames(asset, manifest.settings.scale, manifest.settings.view));
  const rec = { asset: asset.id, category: categoryFolder, ms: Date.now() - t, geometry: geometry.facts, geometryFindings: geometry.findings, files: { ...files, poses: files.poses.map((p: any) => ({ ...p, extent: undefined, ext: p.extent })) }, solids: geometry.solids.filter((s: any) => s.boundary || s.overlapped || s.degenerate || s.nonFinite) };
  appendFileSync(log!, JSON.stringify(rec) + "\n");
  console.error(asset.id, geometry.findings.length, files.findings.length, Date.now() - t, "ms");
}
console.error("total s", (Date.now() - t0) / 1000, "rss MB", Math.round(process.resourceUsage().maxRSS / 1024));
