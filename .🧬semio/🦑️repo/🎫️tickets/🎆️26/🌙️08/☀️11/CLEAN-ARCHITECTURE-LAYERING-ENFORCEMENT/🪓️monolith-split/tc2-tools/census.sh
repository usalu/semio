#!/bin/zsh
# usage: census.sh <tag>
cd /Users/ueli/Documents/semio || exit 1
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
ulimit -n 65536
MANIFESTS=$(tr '\n' ';' < "$F/tc2-tools/${2:-manifests.txt}") DUMP_INPUTS=1 nice -n 10 bun "$F/tc2-tools/tc2-census.ts" > "$F/🗑️generated/tc2-census-$1.log" 2>&1
mkdir -p "$F/🗑️generated/tc2-$1"
cp "$F"/🗑️generated/tc2-census-*.tsv "$F"/🗑️generated/tc2-inputs-* "$F/🗑️generated/tc2-$1/" 2>/dev/null
rm -f "$F"/🗑️generated/tc2-inputs-* 
echo done $?
