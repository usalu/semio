#!/bin/zsh
cd /Users/ueli/Documents/semio
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/lay12-run"
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lay12-tools"
node node_modules/dependency-cruiser/bin/dependency-cruise.mjs "🧰️framework/🔨️modules" "🧰️framework/📦️packages" --config "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs" --output-type json --output-to "$G/$1.json" || exit 1
bun "$T/agg.ts" "$G/$1.json" "$G/$1-agg.json" | head -12
node -e '
const a=JSON.parse(require("fs").readFileSync(process.argv[1],"utf8"));
const cy=a.rows.filter(r=>r.rule==="no-circular");console.log("cycles",cy.length);' "$G/$1-agg.json"
