#!/bin/sh
# Usage: refresh.sh <edges.jsonl>: regenerates SDK entry list and origin tables for every target of the edges file.
cd "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split" || exit 2
node -e '
const fs=require("fs");
const e=Object.entries(JSON.parse(fs.readFileSync("/Users/ueli/Documents/semio/🧰️framework/📦️packages/🟦️typescript/package.json","utf8")).exports).map(([k,v])=>[k,"🧰️framework/📦️packages/🟦️typescript/"+v.slice(2)]);
fs.writeFileSync("🗑️generated/rhu-sdk-entries.json",JSON.stringify(e));
const rows=fs.readFileSync(process.argv[1],"utf8").trim().split("\n").map(l=>JSON.parse(l)).filter(r=>/\.(tsx?|js)$/.test(r.to)&&!r.to.startsWith(".cache"));
fs.writeFileSync("🗑️generated/rhu-orig-list.json",JSON.stringify([...new Set([...e.map(x=>x[1]),...rows.map(r=>r.to)])]));' "$1"
node rhu-tools/origins2.mjs 🗑️generated/rhu-origins3.json 🗑️generated/rhu-orig-list.json
