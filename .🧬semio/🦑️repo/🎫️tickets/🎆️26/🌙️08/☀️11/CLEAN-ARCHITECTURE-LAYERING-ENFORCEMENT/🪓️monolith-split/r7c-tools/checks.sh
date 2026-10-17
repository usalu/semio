#!/bin/zsh
# usage: checks.sh <tag> <plugin dir>  owners scan, taxonomy check, lib-wiring-budget, launch json parse (logs 🗑️generated/r7c-<tag>-*.log)
cd /Users/ueli/Documents/semio || exit 99
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
tag=$1; plugin=$2
ulimit -n 65536
nice -n 10 bun "$F/owners-tools/scan.ts" > "$F/🗑️generated/r7c-$tag-owners.log" 2>&1; echo "owners exit $?"
nice -n 10 bun "$F/🗑️generated/pw0-taxcheck.ts" "$plugin" > "$F/🗑️generated/r7c-$tag-tax.log" 2>&1; echo "taxonomy exit $?"
(cd "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library" && nice -n 10 bun test ./🧪️tests/📏️lib-wiring-budget/🟦️.ts > "/Users/ueli/Documents/semio/$F/🗑️generated/r7c-$tag-wiring.log" 2>&1; echo "wiring exit $?")
bun -e 'for (const f of [".vscode/launch.json", ".vscode/🧩️launch.seed.jsonc"]) { const t = await Bun.file(f).text(); try { Bun.JSONC.parse(t); console.log("parse ok", f); } catch (e) { console.log("parse FAIL", f, String(e).slice(0, 200)); } }'
