/** Usage: bun export.ts <S absolute> <outdir> [ids comma separated] . Exports dark-line and thermal at 1:50, every declared viewpoint, default pose. */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const [root, out, only] = process.argv.slice(2) as [string, string, string | undefined];
const load = (path: string) => import(pathToFileURL(join(root, path)).href);
const { loadCatalogFromFiles, DEFAULT_CATALOG_ROOT } = await load("🔨️modules/🗂️catalog/📥️files/🟦️.ts");
const { evaluate } = await load("🔨️modules/🧮️evaluation/🟦️.ts");
const { defaultDepictionOptions, depict } = await load("🔨️modules/🎨️depiction/🟦️.ts");
const { writeSvg } = await load("🔨️modules/📤️export/🖼️svg/🟦️.ts");

const catalog = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
const wanted = only ? new Set(only.split(",")) : undefined;
mkdirSync(out, { recursive: true });
const index: Record<string, unknown>[] = [];
for (const asset of catalog.assets) {
  if (wanted && !wanted.has(asset.id)) continue;
  const pose = asset.presentation.defaultPose ?? asset.poses[0].id;
  for (const viewpoint of asset.presentation.viewpoints) {
    for (const view of ["dark-line", "thermal"]) {
      try {
        const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: 50 } });
        const base = defaultDepictionOptions(view, "en", 50, viewpoint);
        const scene = depict(state, { ...base, annotations: { ...base.annotations, legend: false } });
        const name = `${asset.category}__${asset.id}__${viewpoint}__${view}.svg`;
        writeFileSync(join(out, name), writeSvg(scene));
        index.push({ id: asset.id, category: asset.category, viewpoint, view, pose, file: name });
      } catch (error) {
        index.push({ id: asset.id, category: asset.category, viewpoint, view, error: String(error) });
      }
    }
  }
}
writeFileSync(join(out, "index.json"), JSON.stringify(index, null, 1));
console.log(`exported ${index.length}`);
