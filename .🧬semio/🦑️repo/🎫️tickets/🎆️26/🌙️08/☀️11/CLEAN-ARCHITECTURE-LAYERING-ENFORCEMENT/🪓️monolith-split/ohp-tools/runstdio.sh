#!/bin/zsh
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
while [ ! -f "$F/🗑️generated/ohp-run-block5d-r1.exit" ]; do sleep 20; done
for n in $(awk '{print $1}' "$F/ohp-tools/stdio-families.txt"); do "$F/ohp-tools/run.sh" $n r1; done
