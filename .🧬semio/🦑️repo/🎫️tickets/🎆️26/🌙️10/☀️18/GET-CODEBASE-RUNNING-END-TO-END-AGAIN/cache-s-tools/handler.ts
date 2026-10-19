import { readFileSync } from "node:fs";
const [script, cmd, n = "14"] = Bun.argv.slice(2);
const src = readFileSync(script, "utf8");
const m = src.match(new RegExp(`["']${cmd}["']\\s*[:,]\\s*([A-Za-z0-9_$]+)`)) ?? src.match(new RegExp(`register\\(\\s*["']${cmd}["']\\s*,\\s*([A-Za-z0-9_$]+)`));
console.log("match:", m?.[0]?.slice(0, 120));
const cls = m?.[1];
if (cls) { const i = src.search(new RegExp(`class ${cls}\\b|function ${cls}\\b|const ${cls}\\b`)); if (i >= 0) console.log(src.slice(i, i + Number(n) * 120).split("\n").slice(0, Number(n)).map(l => l.slice(0, 240)).join("\n")); else console.log("class not local");}
