#!/bin/sh
# Usage: refresh.sh <edges.jsonl> <label>: regenerates SDK entry list and origin tables for every target of the edges file.
cd "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split" || exit 2
node -e '
const fs=require("fs");
const L=process.argv[2];
const e=Object.entries(JSON.parse(fs.readFileSync("/Users/ueli/Documents/semio/🧰️framework/📦️packages/🟦️typescript/package.json","utf8")).exports).map(([k,v])=>[k,"🧰️framework/📦️packages/🟦️typescript/"+v.slice(2)]);
fs.writeFileSync("🗑️generated/rhu2-"+L+"-sdk-entries.json",JSON.stringify(e));
const rows=fs.readFileSync(process.argv[1],"utf8").trim().split("\n").filter(Boolean).map(l=>JSON.parse(l)).filter(r=>/\.(tsx?|js)$/.test(r.to)&&!r.to.startsWith(".cache"));
fs.writeFileSync("🗑️generated/rhu2-"+L+"-orig-list.json",JSON.stringify([...new Set([...e.map(x=>x[1]),...rows.map(r=>r.to)])]));' "$1" "$2"
node rhu2-tools/origins2.mjs 🗑️generated/rhu2-$2-origins.json 🗑️generated/rhu2-$2-orig-list.json
