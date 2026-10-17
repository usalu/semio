#!/bin/zsh
# usage: run-oracle.sh <family> <label> [cfgs]   runs the POS typeck oracle (prefix c4p16) over one c4p16 family
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
cd "$F/pos-tools" || exit 2
fam=$1; label=$2; cfgs=${3:-lib,test}
POS_PREFIX=c4p16 nice python3 pos.py run "$F/c4p16-tools/families/$fam.json" --cfgs $cfgs --label $label > "$F/🗑️generated/c4p16-oracle-$fam-$label.log" 2>&1
echo "fam=$fam label=$label exit=$?" >> "$F/🗑️generated/c4p16-oracle.exit"
