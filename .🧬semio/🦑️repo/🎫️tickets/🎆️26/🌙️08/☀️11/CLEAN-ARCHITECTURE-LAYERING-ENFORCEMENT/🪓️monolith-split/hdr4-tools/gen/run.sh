#!/bin/zsh
# run.sh <family> <label> [cfgs] : single oracle run without re-index (prints the summary)
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G="$F/🗑️generated"
cd /Users/ueli/Documents/semio
python3 "$F/hdr4-tools/pos/pos.py" run "$F/hdr4-tools/pos/families/$1.json" --cfgs "${3:-lib,test}" --label "$2" > "$G/hdr4-run-$1-$2.txt" 2>&1
/usr/bin/grep -vE '^\[|^\s*$|by code|typeck canary|side (cur|base)' "$G/hdr4-run-$1-$2.txt" | cut -c1-300
