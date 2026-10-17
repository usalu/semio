#!/bin/zsh
# usage: checks.sh <tag>   repo checks: launch JSON parse, owners scan, taxonomy (stdio), lib-wiring-budget
cd /Users/ueli/Documents/semio || exit 99
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
tag=$1
ulimit -n 65536
echo "== launch json parse"
bun -e 'const fs=require("fs");let bad=0;for(const f of [".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]){try{Bun.JSONC.parse(fs.readFileSync(f,"utf8"));console.log(f,"parses")}catch(e){bad++;console.log(f,"FAILS",String(e).slice(0,200))}}process.exit(bad?1:0)'
echo "launch exit $?"
echo "== owners scan"
nice -n 10 bun "$F/owners-tools/scan.ts" 2>&1 | tail -8
echo "== taxonomy stdio"
nice -n 10 bun "$F/🗑️generated/pw0-taxcheck.ts" '✏️s/🔌️plugins/🗄️stdio' 2>&1 | tail -15
echo "== lib-wiring-budget"
(cd "$L" && nice -n 10 bun test ./🧪️tests/📏️lib-wiring-budget/🟦️.ts 2>&1 | tail -25)
