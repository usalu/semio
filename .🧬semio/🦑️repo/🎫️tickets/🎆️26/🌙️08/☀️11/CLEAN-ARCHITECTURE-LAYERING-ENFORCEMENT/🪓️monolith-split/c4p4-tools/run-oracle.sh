#!/bin/zsh
# usage: run-oracle.sh <label> <family>...   runs the POS typeck oracle (prefix c4p4) for lib and test
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
cd "$F/c4p4-tools/pos" || exit 2
label=$1; shift
for fam in "$@"; do
  C4P4_OVERLAY=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/c4p4-work/overlay POS_PREFIX=c4p4 python3 pos.py run "$F/c4p4-tools/families/$fam.json" --cfgs lib,test --label $label > "$F/🗑️generated/c4p4-oracle-$label-$fam.log" 2>&1
  echo "fam=$fam label=$label exit=$?" >> "$F/🗑️generated/c4p4-oracle-$label.exit"
done
