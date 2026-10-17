#!/bin/zsh
# usage: tsc.sh <label> -> G/shu-tsc-<label>.txt/.tsv
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/shu-tools/env.sh
cd "$R"
nice -n 10 perl -e 'alarm 1500; exec @ARGV' -- node --max-old-space-size=12000 node_modules/typescript/bin/tsc --noEmit -p "$G/shu-tsconfig.json" > "$G/shu-tsc-$1.txt" 2>&1
echo "exit=$?" > "$G/shu-tsc-$1.exit"
python3 "$T/tscnorm.py" "$G/shu-tsc-$1.txt" "$G/shu-tsc-$1.tsv" >> "$G/shu-tsc-$1.exit"
