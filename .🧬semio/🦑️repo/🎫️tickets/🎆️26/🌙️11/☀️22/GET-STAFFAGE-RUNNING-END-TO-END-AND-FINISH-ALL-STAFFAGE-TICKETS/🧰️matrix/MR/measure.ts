/** 📐️ The checks an asset must keep while its views are refitted, outside the test runner: `bun measure.ts <id|file.json>… [--scales 50,100]` prints per scale the silhouette guard (bounding box per axis, IoU and largest boundary distance per view against the gate of `🧊️catalog-silhouette`), then the issue count of `validateAsset` by code and the findings of the sweep and the consistency rule of `🧊️three-views`. Run it from `S/📦️packages/🟦️typescript`. */
import { parseArgs } from "node:util";
import { join } from "node:path";
import { S, readDoc } from "./lib.ts";

const { measureAsset, VIEWPOINTS } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const { featureOf } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"));
const { validateAsset } = await import(join(S, "📦️packages/🟦️typescript/🟦️.ts"));
const { authoredScales, consistency, sweep } = await import(join(S, "🧪️tests/🧊️three-views/🧰️.ts"));

const { values, positionals } = parseArgs({ allowPositionals: true, options: { scales: { type: "string" } } });
for (const arg of positionals) {
  const asset = readDoc(arg).doc.asset;
  const scales = (values.scales ?? "50,100,200,500").split(",").map(Number).filter((d) => asset.volumes?.[String(d)]?.status === "authored" && asset.representations[String(d)]?.status === "authored");
  const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
  let bad = 0;
  for (const scale of scales) {
    const feature = featureOf(scale);
    const m = measureAsset(asset, scale, undefined, undefined, feature);
    const bb = m.bbox.map((b: number, k: number) => `${b.toFixed(4)}/${dims[k]}${Math.abs(b - dims[k]!) > feature ? (bad++, "!") : ""}`).join(" ");
    const vs = VIEWPOINTS.map((v: string) => `${v} ${m.views[v].iou.toFixed(4)}${m.views[v].iou < 0.92 ? (bad++, "!") : ""}/${(m.views[v].hausdorffM * 1000).toFixed(1)}mm${m.views[v].hausdorffM > 2 * feature ? (bad++, "!") : ""}`).join("  ");
    console.log(`${asset.id} 1:${scale} bbox ${bb}  ${vs}`);
  }
  console.log(`${asset.id} silhouette: ${bad === 0 ? "OK" : `${bad} failing checks`}`);
  const issues = validateAsset(asset) as { code: string; path?: string; message: { en: string } }[];
  const counts = new Map<string, number>();
  for (const issue of issues) counts.set(issue.code, (counts.get(issue.code) ?? 0) + 1);
  console.log(`${asset.id} validate: ${issues.length} issues ${[...counts].map(([c, n]) => `${c}×${n}`).join(", ")}`);
  for (const issue of issues.slice(0, 8)) console.log(`   ${issue.code} ${issue.path ?? ""}: ${issue.message.en}`.slice(0, 300));
  const findings = authoredScales(asset).flatMap((scale: number) => [...sweep(asset, scale), ...consistency(asset, scale)]);
  const checks = new Map<string, number>();
  for (const f of findings) checks.set(f.check, (checks.get(f.check) ?? 0) + 1);
  console.log(`${asset.id} three-views sweep: ${findings.length} findings ${[...checks].map(([c, n]) => `${c}×${n}`).join(", ")}`);
  for (const f of findings.slice(0, 4)) console.log(`   [${f.check}] ${f.message}`.slice(0, 260));
}
