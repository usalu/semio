import fs from "node:fs";
const used = new Map(JSON.parse(fs.readFileSync(process.argv[2], "utf8")));
for (const e of process.argv.slice(3)) { const k = e.replace(/️/g, ""); console.log(e, used.get(k) ?? "UNUSED", /^\p{RGI_Emoji}$/v.test(e) || /^\p{RGI_Emoji}$/v.test(e + "️") ? "rgi" : "NOT-RGI"); }
