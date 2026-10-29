import { readFileSync } from "node:fs";
const { rustTokens } = await import(process.argv[2]);
for (const f of process.argv.slice(3)) { const t = rustTokens(readFileSync(f, "utf8")); const c = {}; for (const x of t) if ("(){}[]".includes(x.text) && x.text.length===1) c[x.text] = (c[x.text]||0)+1; console.log(f.slice(-60), JSON.stringify(c)); }
