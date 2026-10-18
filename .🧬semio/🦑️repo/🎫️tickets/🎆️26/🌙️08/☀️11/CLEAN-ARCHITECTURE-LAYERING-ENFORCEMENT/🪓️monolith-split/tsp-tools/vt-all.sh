#!/bin/zsh
cd /Users/ueli/Documents/semio
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
i=0
while IFS= read -r cfg; do
  i=$((i+1))
  python3 $F/tsp-tools/vt.py "$cfg" "fin$i" 2>&1 | head -30
  echo "CFG $i $cfg"
done < $F/tsp-tools/vitest-configs-final.txt
