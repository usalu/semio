/** 🧱️ Builds a tsconfig over every TypeScript file this batch touched or created (taken from the batch snapshot). Usage: bun mktsc.ts <batch-snapshot.json> <out.json> */
import { readFileSync, writeFileSync, existsSync } from "node:fs";
const ROOT = "/Users/ueli/Documents/semio";
const snap = JSON.parse(readFileSync(process.argv[2], "utf8")) as Record<string, string | null>;
const files = Object.keys(snap).filter(f => /\.(ts|tsx)$/u.test(f)).map(f => ROOT + "/" + f).filter(f => existsSync(f)).sort();
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
  files,
};
writeFileSync(process.argv[3], JSON.stringify(cfg, null, 1));
console.log(`files=${files.length}`);
