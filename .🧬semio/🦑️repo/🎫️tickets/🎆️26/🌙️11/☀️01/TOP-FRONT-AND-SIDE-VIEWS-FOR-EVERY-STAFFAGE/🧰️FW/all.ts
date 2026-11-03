import { writeFileSync } from "node:fs";
import { checkAssetDetailed } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts";
import { runPreview } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";
const out: string[] = [];
const ids = (process.env.FW_CHECK ?? "").split(",").filter(Boolean);
const { loadCatalogFromFiles, DEFAULT_CATALOG_ROOT } = await import("../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts");
const catalog = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
for (const id of ids) {
  const asset = catalog.assets.find((a) => a.id === id)!;
  const report = checkAssetDetailed(asset, { samples: 3 });
  out.push(`${id}: ${report.problems.length ? report.problems.slice(0, 12).join("\n   ") : "ok"}`);
}
if (process.env.FW_PREVIEW) {
  const files = await runPreview(["--asset", process.env.FW_PREVIEW, "--scale", "50", "--views", "--dpi", "150", "--out", process.env.FW_OUT!]);
  out.push(...files);
}
writeFileSync(process.env.FW_LOG!, out.join("\n") + "\n");
