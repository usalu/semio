#!/bin/zsh
# usage: orc.sh <family> <label> [cfgs]  -> runs the companion-aware oracle and prints the NEW diagnostics
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
T="$F/ttc-tools"
fam=$1; label=$2; cfgs=${3:-lib,test}
perl -e 'alarm 1500; exec @ARGV' -- python3 "$T/ttc.py" oracle "$T/families/$fam.json" --cfgs $cfgs --label $label --iters 4 > "$F/🗑️generated/ttc-run-$fam-$label.log" 2>&1
/usr/bin/grep -E '^-- cfg' "$F/🗑️generated/ttc-run-$fam-$label.log" | cut -c1-200
