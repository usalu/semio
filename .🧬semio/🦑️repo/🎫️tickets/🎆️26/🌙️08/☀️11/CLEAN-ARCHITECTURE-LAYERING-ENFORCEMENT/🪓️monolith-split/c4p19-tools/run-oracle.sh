#!/bin/zsh
# usage: run-oracle.sh <fam letter> <label> [cfgs]   runs the POS typeck oracle (prefix c4p19) over one c4p19 family
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
cd "$F/pos-tools" || exit 2
fam=$1; label=$2; cfgs=${3:-lib,test}
POS_PREFIX=c4p19 python3 pos.py run "$F/c4p19-tools/families/c4p19$fam.json" --cfgs $cfgs --label $label > "$F/🗑️generated/c4p19-oracle-$label-$fam.log" 2>&1
echo "fam=$fam label=$label exit=$?" >> "$F/🗑️generated/c4p19-oracle.exit"
