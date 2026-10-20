import { readFileSync, writeFileSync } from "node:fs";
import { dirname } from "node:path";
const WS = "/Users/ueli/Documents/semio", C = process.argv[2];
const rows = JSON.parse(readFileSync(`${C}/🗑️generated/cachefix-x/fallthrough.json`, "utf8"));
const want = new Map<string, { project: string; root: string; targets: Set<string> }>();
const add = (file: string, project: string, target: string) => { const root = dirname(file); const e = want.get(file) ?? { project, root, targets: new Set() }; e.targets.add(target); want.set(file, e); };
for (const r of rows) { if (/-check$/.test(r.name) || /generator/.test(r.name) || r.name === "preview-generated") continue; add(r.file, r.project, r.name); }
const extra = ["build", "build-lean", "generate", "build-inputs", "build-viz", "generate-viz", "generate-font-metrics", "schema-mirror", "generate-schema-index", "generate-distribution", "generate-scale-fixture", "generate-wgpu", "generate-jco-package-adapter", "generate-taxonomy-census", "generate-taxonomy-duplicates", "schema-audit", "schema-compile", "schema-entries", "schema-oracle", "schema-parsers-report", "schema-test", "schema-verify", "artifact-package-contract", "toolchain", "stdio-codec", "stdio-cross-platform", "stdio-exhaustive", "stdio-inference", "stdio-long", "stdio-mutation-law", "stdio-quick", "stdio-runtime", "stdio-standards-coverage", "stdio-schema-parity", "stdio-document-contract", "stdio-document-contract-native", "schema-fixture-boundary"];
const list = readFileSync(`${C}/🗑️generated/cachefix-x/pj-list.txt`, "utf8").split("\n").filter(Boolean);
for (const file of list) { let j: any; try { j = JSON.parse(readFileSync(`${WS}/${file}`, "utf8")); } catch { continue; }
  for (const name of extra) { const t = j.targets?.[name]; if (!t || t.cache === false) continue; if (t.outputs?.length) continue; if (t.metadata?.semio?.rustBuildIntent) continue; add(file, j.name, name); } }
const out = [...want.values()].map((e) => ({ project: e.project, root: e.root, targets: [...e.targets] }));
writeFileSync(`${C}/🗑️generated/cachefix-x/targets-1.json`, JSON.stringify(out, null, 1));
console.log(out.length, out.reduce((a, e) => a + e.targets.length, 0));
