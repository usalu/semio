// 🩺 Read-only run of the staffage validateManifest over the catalog manifest and every asset file; prints the issue codes per row.
import { readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";

const [moduleFile, catalogDir] = process.argv.slice(2);
const { validateManifest } = await import(moduleFile!);
const manifest = JSON.parse(readFileSync(join(catalogDir!, "🔣️.json"), "utf8"));
const folderOf = new Map<string, string>(manifest.categories.map((c: { id: string; folder: string }) => [c.id, c.folder]));
const assets: unknown[] = [];
for (const row of manifest.entries as { id: string; category: string }[]) {
  const dir = join(catalogDir!, folderOf.get(row.category)!, row.id);
  const file = readdirSync(dir).find((n) => n.endsWith(".json") && statSync(join(dir, n)).isFile());
  assets.push(JSON.parse(readFileSync(join(dir, file!), "utf8")).asset);
}
const issues = validateManifest(manifest, assets) as { path: string; code: string; message: { en: string } }[];
console.log(JSON.stringify({ manifestIssues: issues.length, codes: [...new Set(issues.map((i) => i.code))] }));
for (const issue of issues) console.log(`${issue.code} ${issue.path}: ${issue.message.en}`);

const SEMANTICS = ["true-extent", "true-position", "nominal"];
const VIEWS = ["elevation", "front", "plan"];
const symbolMismatches: string[] = [];
for (let i = 0; i < manifest.entries.length; i++) {
  const row = manifest.entries[i] as { id: string; symbols: string[]; representations: Record<string, string> };
  const asset = assets[i] as { id: string; symbols: { id: string; semantics: string; denominators: number[]; geometry: Record<string, unknown> }[]; representations: Record<string, { status: string; denominator: number; alternatives?: { kind: string; symbol?: string; denominator?: number }[] }> };
  const assetIds = asset.symbols.map((s) => s.id);
  for (const id of row.symbols) if (!assetIds.includes(id)) symbolMismatches.push(`${row.id}: manifest symbol ${id} is not in the asset file`);
  for (const id of assetIds) if (!row.symbols.includes(id)) symbolMismatches.push(`${row.id}: asset symbol ${id} is not in the manifest row`);
  for (const symbol of asset.symbols) {
    if (!SEMANTICS.includes(symbol.semantics)) symbolMismatches.push(`${row.id}: ${symbol.id} semantics ${symbol.semantics} is not one of ${SEMANTICS.join(", ")}`);
    const missing = VIEWS.filter((v) => !(v in symbol.geometry));
    if (missing.length) symbolMismatches.push(`${row.id}: ${symbol.id} lacks views ${missing.join(", ")}`);
    if (!symbol.denominators.length) symbolMismatches.push(`${row.id}: ${symbol.id} offers no scales`);
    for (const band of Object.values(asset.representations)) {
      for (const alternative of band.alternatives ?? []) {
        if (alternative.kind === "symbol" && alternative.symbol === symbol.id && !symbol.denominators.includes(band.denominator)) {
          symbolMismatches.push(`${row.id}: alternative at 1:${band.denominator} points to ${symbol.id}, which offers ${symbol.denominators.join("/")}`);
        }
      }
    }
  }
}
console.log(JSON.stringify({ symbolMismatches: symbolMismatches.length }));
for (const line of symbolMismatches) console.log(`symbol ${line}`);
