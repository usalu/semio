/** 🧱️ hr2 tsconfig builder over a file list. Usage: bun mktsc.ts <list.txt> <out.json> */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
const [listPath, outPath] = process.argv.slice(2);
const ROOT = "/Users/ueli/Documents/semio";
const all = readFileSync(listPath!, "utf8").split("\n").filter((x) => x.length > 0).map((f) => `${ROOT}/${f}`).filter((f) => existsSync(f)).sort();
const cfg = {
  extends: `${ROOT}/tsconfig.json`,
  compilerOptions: {
    allowImportingTsExtensions: true, noEmit: true, skipLibCheck: true, incremental: false, types: ["bun", "node"],
    paths: {
      "dom-accessibility-api": [`${ROOT}/node_modules/dom-accessibility-api/dist/index.d.ts`],
      "fast-json-stable-stringify": [`${ROOT}/🧰️framework/📦️packages/🟦️typescript/🌿️ambient/🔤️fast-json-stable-stringify/🟦️.d.ts`],
      "lines-and-columns": [`${ROOT}/node_modules/lines-and-columns/build/index.d.ts`],
    },
  },
  include: [],
  files: all,
};
writeFileSync(outPath!, JSON.stringify(cfg, null, 1));
console.log(`files=${all.length}`);
