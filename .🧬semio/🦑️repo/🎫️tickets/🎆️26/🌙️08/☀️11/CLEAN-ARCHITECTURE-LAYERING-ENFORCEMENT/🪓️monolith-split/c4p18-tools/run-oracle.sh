#!/bin/zsh
# usage: run-oracle.sh <label> [cfgs] [families...]   runs the POS typeck oracle (prefix c4p18) over the four c4p18 family files, one after the other
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
cd "$F/pos-tools" || exit 2
label=$1; cfgs=${2:-lib,test}; shift 2
fams=("$@"); [ ${#fams[@]} -eq 0 ] && fams=(c4p18g3 c4p18gi c4p18vc c4p18an)
for fam in $fams; do
  POS_PREFIX=c4p18 perl -e 'alarm 3000; exec @ARGV' -- python3 pos.py run "$F/c4p18-tools/families/$fam.json" --cfgs $cfgs --label $label > "$F/🗑️generated/c4p18-pos-$fam-$label.log" 2>&1
  echo "fam=$fam label=$label exit=$?" >> "$F/🗑️generated/c4p18-oracle.exit"
done
