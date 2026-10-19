import { readFileSync } from "node:fs";
const all: string[] = JSON.parse(readFileSync("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/tjcov-listing.json", "utf8"));
const re = new RegExp(process.argv[2]!);
for (const f of all) { if (process.argv[3] && !f.includes(process.argv[3])) continue; const t = readFileSync("/Users/ueli/Documents/semio/" + f, "utf8"); const m = t.match(re); if (m) console.log(f.split("/").slice(-6).join("/"), "|", m[0].slice(0, 200)); }
