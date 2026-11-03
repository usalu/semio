import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2]!;
let t = readFileSync(file, "utf8");
const sub = (from: string, to: string): void => {
  if (!t.includes(from)) throw new Error(`missing: ${from.slice(0, 60)}`);
  t = t.replace(from, to);
};
sub('import { validateAsset, validateRanges, validateSymbols }', 'import { validateAsset, validateManifest, validateRanges, validateSymbols }');
sub("const result = { generatedBy", `for (const issue of validateManifest(catalog.manifest, catalog.assets)) {
  const index = Number(/entries\[(\d+)\]/u.exec(issue.path)?.[1] ?? -1);
  const entry = catalog.manifest.entries[index];
  const key = entry === undefined ? "manifest" : \`\${entry.category}/\${entry.id}\`;
  const slot = (assets[key] ??= { failures: 0, codes: {}, issues: [] });
  slot.failures++;
  slot.codes[issue.code] = (slot.codes[issue.code] ?? 0) + 1;
  slot.issues.push({ code: issue.code, path: issue.path, message: issue.message.en });
  codeTotals[issue.code] = (codeTotals[issue.code] ?? 0) + 1;
  (codeAssets[issue.code] ??= []).includes(key) || codeAssets[issue.code]!.push(key);
}
const result = { generatedBy`);
writeFileSync(file, t);
