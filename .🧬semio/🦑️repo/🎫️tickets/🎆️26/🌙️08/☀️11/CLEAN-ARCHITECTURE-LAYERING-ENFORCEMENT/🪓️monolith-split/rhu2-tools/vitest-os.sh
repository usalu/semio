#!/bin/sh
# 🧪️ Runs the os vitest project (in-source suites) once. Usage: vitest-os.sh <label>
L="$1"
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
cd /Users/ueli/Documents/semio || exit 2
perl -e 'alarm 1500; exec @ARGV' -- bun --bun node_modules/vitest/vitest.mjs run --config "🧰️framework/🛍️products/💻️os/🧪️tests/🎚️config/🟦️.ts" --reporter=json --outputFile="$G/rhu2-os-vitest-$L.json" > "$G/rhu2-os-vitest-$L.txt" 2>&1
echo $? > "$G/rhu2-os-vitest-$L.status"
