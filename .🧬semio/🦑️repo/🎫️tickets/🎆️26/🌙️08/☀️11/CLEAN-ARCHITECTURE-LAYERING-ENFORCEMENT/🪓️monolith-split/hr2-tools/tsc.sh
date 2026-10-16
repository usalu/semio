#!/bin/sh
# Runs tsc --noEmit for a generated tsconfig. Usage: tsc.sh <label> <tsconfig>
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
cd /Users/ueli/Documents/semio || exit 2
date +%s > "$G/hr2-tsc-$1-start.txt"
perl -e "alarm 2400; exec @ARGV" -- node --max-old-space-size=8192 node_modules/typescript/bin/tsc -p "$2" --noEmit --pretty false > "$G/hr2-tsc-$1.txt" 2> "$G/hr2-tsc-$1.err"
echo $? > "$G/hr2-tsc-$1-status.txt"
date +%s > "$G/hr2-tsc-$1-end.txt"
