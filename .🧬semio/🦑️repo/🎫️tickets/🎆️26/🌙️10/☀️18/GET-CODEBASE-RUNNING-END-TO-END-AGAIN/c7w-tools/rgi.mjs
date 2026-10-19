import {readFileSync} from "node:fs";
const names = readFileSync(process.argv[2], "utf8").split("\n").filter(Boolean);
let bad = 0;
for (const n of names) { const i = n.indexOf("️"); const base = i >= 0 ? n.slice(0, i) : n.slice(0, 1); const ok = i >= 0 && (/^\p{RGI_Emoji}$/v.test(base) || /^\p{RGI_Emoji}$/v.test(base + "️")); if (!ok) bad += 1; console.log(ok ? "RGI" : "NOT-RGI", JSON.stringify(base), n); }
console.log(`bad=${bad}`);
