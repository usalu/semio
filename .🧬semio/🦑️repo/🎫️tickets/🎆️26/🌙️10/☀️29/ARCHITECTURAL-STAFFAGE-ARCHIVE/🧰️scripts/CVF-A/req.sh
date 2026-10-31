#!/bin/bash
# usage: req.sh <name> <asset-file> [physics]   -> prints result
G="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/CVF-A/${SRV:-srv2}"
name=$1; file=$2; phys=${3:-false}
rm -f "$G/res/$name.txt"
printf '{"file":"%s","physics":%s,"samples":4,"limit":%s}' "$file" "$phys" "${LIMIT:-40}" > "$G/req/$name.json.tmp"
mv "$G/req/$name.json.tmp" "$G/req/$name.json"
for i in $(seq 1 ${WAIT:-240}); do [ -f "$G/res/$name.txt" ] && break; sleep 1; done
cat "$G/res/$name.txt" 2>/dev/null || echo "no result yet"
