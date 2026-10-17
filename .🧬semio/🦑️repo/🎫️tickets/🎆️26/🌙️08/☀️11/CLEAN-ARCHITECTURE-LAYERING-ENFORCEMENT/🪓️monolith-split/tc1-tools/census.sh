#!/bin/zsh
# usage: census.sh <tag>
cd /Users/ueli/Documents/semio || exit 1
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
ulimit -n 65536
MANIFESTS=$(tr '\n' ';' < "$F/tc1-tools/manifests.txt") DUMP_INPUTS=1 nice -n 10 bun "$F/tc1-tools/tc1-census.ts" > "$F/🗑️generated/tc1-census-$1.log" 2>&1
mkdir -p "$F/🗑️generated/tc1-$1"
cp "$F"/🗑️generated/tc1-census-*.tsv "$F"/🗑️generated/tc1-inputs-* "$F/🗑️generated/tc1-$1/" 2>/dev/null
echo done $?
