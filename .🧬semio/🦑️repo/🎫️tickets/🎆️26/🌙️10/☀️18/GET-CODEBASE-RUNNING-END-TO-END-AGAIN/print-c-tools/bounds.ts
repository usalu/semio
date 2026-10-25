import { build } from "esbuild";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const root = "/Users/ueli/Documents/semio", product = join(root, "🧰️framework/🛍️products/📓️print");
const contract = JSON.parse(readFileSync(join(product, "🎮️commands/🧪️print-pipeline-verification/🧪️tests/🧫️fixtures/🧫️command-boundaries.json"), "utf8"));
const out = process.argv[2]!;
const res: Record<string, unknown> = {};
for (const entry of contract.entries as string[]) {
  const r = await build({ absWorkingDir: root, entryPoints: [join(product, entry)], platform: "node", format: "esm", bundle: true, packages: "external", external: ["bun"], write: false, metafile: true, logLevel: "silent" });
  const files = Object.keys(r.metafile!.inputs).sort();
  const expected: string[] = contract.sourceInputs[entry];
  const edges: Record<string, string[]> = {};
  for (const [f, v] of Object.entries(r.metafile!.inputs)) edges[f] = v.imports.map(i => i.path);
  res[entry] = { files: files.length, expected: expected.length, extra: files.filter(f => !expected.includes(f)), missing: expected.filter(f => !files.includes(f)), edges };
}
writeFileSync(out, JSON.stringify(res));
console.log(Object.entries(res).map(([k, v]: any) => `${k} actual=${v.files} expected=${v.expected} extra=${v.extra.length} missing=${v.missing.length}`).join("\n"));
