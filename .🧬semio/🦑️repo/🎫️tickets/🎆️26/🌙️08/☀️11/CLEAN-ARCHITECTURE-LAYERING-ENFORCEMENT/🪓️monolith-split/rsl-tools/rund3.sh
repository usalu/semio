#!/bin/bash
# 🏃️ Runs the POS oracle on the store union (no baseline revision) with label $1, configurations $2 (default lib).
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
label="$1"; cfgs="${2:-lib}"
cd "$F/pos-tools" && POS_PREFIX=rsl nice python3 pos.py run "$F/rsl-tools/families/rsld3.json" --cfgs "$cfgs" --no-base --label "$label" > "$F/🗑️generated/rsl-d3-$label-console.log" 2>&1
echo "rsld3 $label rc=$?" >> "$F/🗑️generated/rsl-rund3.status"
