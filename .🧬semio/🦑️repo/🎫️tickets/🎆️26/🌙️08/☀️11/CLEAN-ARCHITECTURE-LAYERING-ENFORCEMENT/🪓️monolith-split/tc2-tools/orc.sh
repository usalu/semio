#!/bin/zsh
# usage: orc.sh <family> <label> [cfgs]
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
T="$F/tc2-tools/ttc"
fam=$1; label=$2; cfgs=${3:-lib,test}
ulimit -n 65536
nice -n 10 perl -e 'alarm 3300; exec @ARGV' -- python3 "$T/ttc.py" oracle "$T/families/$fam.json" --cfgs $cfgs --label $label --iters 4 > "$F/🗑️generated/tc2-run-$fam-$label.log" 2>&1
/usr/bin/grep -E '^-- cfg|NEW|error' "$F/🗑️generated/tc2-run-$fam-$label.log" | cut -c1-220 | head -20
