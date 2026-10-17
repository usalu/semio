import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
const { rustTokens, rustTokenPairs } = await import("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts");
const bad: string[] = [];
const walk = (d: string): void => { for (const e of readdirSync(d)) { const p = join(d, e); if (statSync(p).isDirectory()) walk(p); else if (p.endsWith(".rs")) { const t = rustTokens(readFileSync(p, "utf8")); const pairs = rustTokenPairs(t); if (t.some((tok: any, i: number) => ["(", ")", "[", "]", "{", "}"].includes(tok.text) && !pairs.has(i))) bad.push(p); } } };
for (const d of process.argv.slice(2)) walk(d);
console.log("unbalanced:", bad.length); for (const b of bad) console.log(b);
