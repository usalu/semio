#!/bin/zsh
# usage: run-oracle.sh <label> [cfgs]   runs the POS typeck oracle (prefix c4p34, absolute runs) over the c4p34 family
export PATH="/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/usr/local/bin:$HOME/.bun/bin:$HOME/.cargo/bin"
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
cd "$F/pos-tools" || exit 2
label=$1; cfgs=${2:-lib,test}
POS_PREFIX=c4p34 perl -e 'alarm 3000; exec @ARGV' -- python3 pos.py run "$F/c4p34-tools/families/c4p34.json" --cfgs $cfgs --no-base --label $label > "$F/🗑️generated/c4p34-pos-$label.log" 2>&1
echo "label=$label exit=$?" >> "$F/🗑️generated/c4p34-oracle.exit"
