#!/bin/zsh
# usage: run-oracle.sh <label> [cfgs]   runs the POS typeck oracle (prefix c4p17) over the c4p17 family
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
cd "$F/pos-tools" || exit 2
label=$1; cfgs=${2:-lib,test}
POS_PREFIX=c4p17 python3 "$F/c4p17-tools/c4p17-pos-run.py" run "$F/c4p17-tools/families/c4p17.json" --cfgs $cfgs --label $label > "$F/🗑️generated/c4p17-oracle-$label.log" 2>&1
echo "label=$label exit=$?" >> "$F/🗑️generated/c4p17-oracle.exit"
