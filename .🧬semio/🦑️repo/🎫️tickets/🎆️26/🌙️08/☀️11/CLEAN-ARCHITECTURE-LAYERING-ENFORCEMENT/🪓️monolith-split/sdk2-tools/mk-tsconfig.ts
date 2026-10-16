/** 🧱️ Writes a tsconfig whose `files` are the union of the given list files (repo-relative paths, one per line) plus all current SDK facades. Usage: bun mk-tsconfig.ts <out.json> <list.txt>... */
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { execSync } from "node:child_process";
const ROOT = "/Users/ueli/Documents/semio";
const [out, ...lists] = process.argv.slice(2);
const files = new Set<string>();
for (const l of lists) for (const f of readFileSync(l, "utf8").split("\n").map(x => x.trim()).filter(Boolean)) if (/\.(ts|tsx)$/.test(f) && existsSync(ROOT + "/" + f)) files.add(f);
const sdk = JSON.parse(readFileSync(`${ROOT}/🧰️framework/📦️packages/🟦️typescript/package.json`, "utf8")).exports as Record<string, string>;
for (const t of Object.values(sdk)) files.add(("🧰️framework/📦️packages/🟦️typescript/" + t.slice(2)).normalize("NFC"));
const cfg = {
  extends: ROOT + "/tsconfig.json",
  compilerOptions: { allowImportingTsExtensions: true, noEmit: true, skipLibCheck: true, incremental: false, types: ["bun", "node"],
    paths: { "dom-accessibility-api": [ROOT + "/node_modules/dom-accessibility-api/dist/index.d.ts"], "fast-json-stable-stringify": [ROOT + "/🧰️framework/📦️packages/🟦️typescript/🌿️ambient/🔤️fast-json-stable-stringify/🟦️.d.ts"], "lines-and-columns": [ROOT + "/node_modules/lines-and-columns/build/index.d.ts"] } },
  files: [...files].sort().map(f => ROOT + "/" + f),
};
writeFileSync(out, JSON.stringify(cfg, null, 1));
console.log(`files=${cfg.files.length}`);
