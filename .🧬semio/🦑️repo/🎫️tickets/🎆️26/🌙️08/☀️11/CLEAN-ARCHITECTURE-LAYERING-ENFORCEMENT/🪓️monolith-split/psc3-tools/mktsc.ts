/** 🧱️ Builds a tsconfig over the files the codemod touches (plus optional facades). Usage: bun mktsc.ts <report.json> <out.json> [--facades <plan.json>] */
import { readFileSync, writeFileSync, existsSync } from "node:fs";
const args = process.argv.slice(2);
const ROOT = "/Users/ueli/Documents/semio";
const report = JSON.parse(readFileSync(args[0], "utf8"));
const files = Object.keys(report.files).filter(f => /\.(ts|tsx)$/u.test(f));
const fi = args.indexOf("--facades");
const extra: string[] = [];
if (fi >= 0) { const plan = JSON.parse(readFileSync(args[fi + 1], "utf8")); for (const f of plan.facades) extra.push(f.facade); }
const all = [...new Set([...files, ...extra])].map(f => ROOT + "/" + f).filter(f => existsSync(f)).sort();
const cfg = {
  extends: ROOT + "/tsconfig.json",
  compilerOptions: {
    allowImportingTsExtensions: true, noEmit: true, skipLibCheck: true, incremental: false, types: ["bun", "node"],
    paths: {
      "dom-accessibility-api": [ROOT + "/node_modules/dom-accessibility-api/dist/index.d.ts"],
      "fast-json-stable-stringify": [ROOT + "/🧰️framework/📦️packages/🟦️typescript/🌿️ambient/🔤️fast-json-stable-stringify/🟦️.d.ts"],
      "lines-and-columns": [ROOT + "/node_modules/lines-and-columns/build/index.d.ts"],
    },
  },
  files: all,
};
writeFileSync(args[1], JSON.stringify(cfg, null, 1));
console.log(`files=${all.length}`);
