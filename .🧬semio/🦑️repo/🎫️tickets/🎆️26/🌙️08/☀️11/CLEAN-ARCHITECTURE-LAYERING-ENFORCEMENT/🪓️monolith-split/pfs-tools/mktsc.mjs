// usage: node mktsc.mjs <out.json> <file...|list.txt>  (repo-relative or absolute)
import fs from "node:fs";
import path from "node:path";
const REPO = "/Users/ueli/Documents/semio";
const [out, ...rest] = process.argv.slice(2);
const files = rest.flatMap((a) => (a.endsWith(".txt") ? fs.readFileSync(a, "utf8").split("\n").filter(Boolean) : [a]));
const abs = [...new Set(files.map((f) => path.resolve(REPO, f)).filter((f) => fs.existsSync(f)))];
fs.writeFileSync(out, JSON.stringify({ extends: `${REPO}/tsconfig.json`, compilerOptions: { noEmit: true, incremental: false, types: ["bun", "node", "react", "react-dom", "vite/client"] }, include: [], files: abs }, null, 1));
console.log(abs.length + " files");
