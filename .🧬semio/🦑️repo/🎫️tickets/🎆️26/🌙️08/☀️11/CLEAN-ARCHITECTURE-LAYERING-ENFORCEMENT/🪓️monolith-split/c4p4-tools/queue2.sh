#!/bin/zsh
# resumed queue after the session restart: raster check-applied, typeck oracle, census probe, draw A/B gate
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
cd /Users/ueli/Documents/semio || exit 2
bun ./📜️script.ts new crates '✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/📦️packages/🦀️rust' --check-applied --base=f856dff0813 --sandbox=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/c4p4-work/ca-after-r > "$F/🗑️generated/c4p4-checkapplied-after-raster.log" 2>&1; echo "raster exit=$?" >> "$F/🗑️generated/c4p4-checkapplied-after.exit"
"$F/c4p4-tools/run-oracle.sh" post1 c4p4c c4p4b c4p4a
perl -e 'alarm 3000; exec @ARGV' -- bun "$F/c4p4-tools/census-probe.ts" > "$F/🗑️generated/c4p4-census-probe2.json" 2> "$F/🗑️generated/c4p4-census-probe2.err"
cd "$F/c4p4-tools" && C4P4_SCR=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/c4p4-work python3 ab.py draw drawing > "$F/🗑️generated/c4p4-ab-draw.log" 2>&1
echo queue2-done >> "$F/🗑️generated/c4p4-queue2.exit"
