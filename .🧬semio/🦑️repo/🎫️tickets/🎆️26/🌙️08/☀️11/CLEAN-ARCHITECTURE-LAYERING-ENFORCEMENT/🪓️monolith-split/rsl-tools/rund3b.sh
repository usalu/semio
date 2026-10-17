#!/bin/bash
# 🏃️ Runs the POS oracle on the store union against the HEAD revision, configurations $2 (default lib,test), label $1.
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
label="$1"; cfgs="${2:-lib,test}"
cd "$F/pos-tools" && POS_PREFIX=rsl nice python3 pos.py run "$F/rsl-tools/families/rsld3b.json" --cfgs "$cfgs" --label "$label" > "$F/🗑️generated/rsl-d3b-$label-console.log" 2>&1
echo "rsld3b $label rc=$?" >> "$F/🗑️generated/rsl-rund3.status"
