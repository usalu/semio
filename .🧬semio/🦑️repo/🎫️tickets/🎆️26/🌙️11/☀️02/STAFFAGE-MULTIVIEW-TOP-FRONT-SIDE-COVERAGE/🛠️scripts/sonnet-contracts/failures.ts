/** Usage: bun failures.ts <out.json> [--ranges] [--shard i/n] [--category id]
 * Validates the catalog (and with --ranges the print suitability at the nominal scale and both ends of every valid range).
 * --shard i/n takes the assets whose index modulo n is i (0-based); the manifest is checked by shard 0 only; merge shards with merge.ts. */
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { DEFAULT_PRINT_PROFILE } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { DEFAULT_CATALOG_ROOT, loadCatalogFromFiles, loadEnvironments } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { validateAsset, validateManifest, validateRanges, validateSymbols } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";

type Entry = { failures: number; codes: Record<string, number>; issues: { code: string; path: string; message: string; data?: unknown }[] };

const argument = (name: string): string | undefined => {
  const at = process.argv.indexOf(name);
  return at < 0 ? undefined : process.argv[at + 1];
};
const out = resolve(process.argv[2]!);
const shardText = argument("--shard");
const [shard, shards] = shardText === undefined ? [0, 1] : shardText.split("/").map(Number) as [number, number];
const onlyCategory = argument("--category");
const ranges = process.argv.includes("--ranges");

const catalog = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
const known = new Map(catalog.assets.map((asset) => [asset.id, asset] as const));
const environments = new Set(((await loadEnvironments(catalog.root))?.presets ?? []).map((preset) => preset.id));
const assets: Record<string, Entry> = {};
const crashed: Record<string, string> = {};
const selected = catalog.loaded.filter((entry, index) => index % shards === shard && (onlyCategory === undefined || entry.asset.category === onlyCategory));

const record = (key: string, issue: { code: string; path: string; message: { en: string }; data?: unknown }): void => {
  const slot = (assets[key] ??= { failures: 0, codes: {}, issues: [] });
  slot.failures++;
  slot.codes[issue.code] = (slot.codes[issue.code] ?? 0) + 1;
  slot.issues.push({ code: issue.code, path: issue.path, message: issue.message.en, ...(issue.data === undefined ? {} : { data: issue.data }) });
};

for (const entry of selected) {
  const key = `${entry.asset.category}/${entry.asset.id}`;
  console.error(`[DEBUG] shard ${shard}/${shards} ${key}`);
  try {
    for (const issue of [...validateAsset(entry.asset, { assets: known, environments }), ...validateSymbols(entry.asset, DEFAULT_PRINT_PROFILE), ...(ranges ? validateRanges(entry.asset, DEFAULT_PRINT_PROFILE) : [])]) record(key, issue);
  } catch (error) {
    crashed[key] = (error as Error).message;
  }
}
if (shard === 0 && onlyCategory === undefined) {
  for (const issue of validateManifest(catalog.manifest, catalog.assets)) {
    const index = Number(/entries\[(\d+)\]/u.exec(issue.path)?.[1] ?? -1);
    const entry = catalog.manifest.entries[index];
    record(entry === undefined ? "manifest" : `${entry.category}/${entry.id}`, issue);
  }
}

const codeTotals: Record<string, number> = {};
const assetsPerCode: Record<string, number> = {};
for (const entry of Object.values(assets)) for (const [code, n] of Object.entries(entry.codes)) {
  codeTotals[code] = (codeTotals[code] ?? 0) + n;
  assetsPerCode[code] = (assetsPerCode[code] ?? 0) + 1;
}
const result = { generatedBy: "sonnet-contracts", shard: `${shard}/${shards}`, assetsChecked: selected.length, assetsFailing: Object.keys(assets).length, assetsCrashed: Object.keys(crashed).length, codeTotals, assetsPerCode, crashed, assets };
mkdirSync(dirname(out), { recursive: true });
writeFileSync(out, JSON.stringify(result, null, 1));
console.log(JSON.stringify({ shard: result.shard, checked: result.assetsChecked, failing: result.assetsFailing, crashed: result.assetsCrashed, codeTotals, assetsPerCode }, null, 1));
