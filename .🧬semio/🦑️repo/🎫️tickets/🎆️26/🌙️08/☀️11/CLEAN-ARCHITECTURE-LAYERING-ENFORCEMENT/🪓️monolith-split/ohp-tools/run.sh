#!/bin/zsh
# usage: run.sh <family> <label> [cfgs]   runs pos oracle for ohp-tools/families/<family>.json (no baseline), log F/🗑️generated/ohp-run-<family>-<label>.log
cd /Users/ueli/Documents/semio || exit 99
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
fam=$1; label=$2; cfgs=${3:-lib,test}
ulimit -n 65536
export POS_PREFIX=ohp
nice -n 10 python3 "$F/ohp-tools/pos/pos.py" run "$F/ohp-tools/families/$fam.json" --cfgs $cfgs --no-base --label $label > "$F/🗑️generated/ohp-run-$fam-$label.log" 2>&1
echo $? > "$F/🗑️generated/ohp-run-$fam-$label.exit"
