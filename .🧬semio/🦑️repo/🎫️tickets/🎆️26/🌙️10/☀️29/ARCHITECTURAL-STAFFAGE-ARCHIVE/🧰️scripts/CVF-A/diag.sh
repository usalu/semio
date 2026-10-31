#!/bin/bash
# usage: diag.sh <name> <file> <scales json> [draw=true] [zoom] [poses json]
G="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/CVF-A/${SRV:-srv2}"
O="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/CVF-A/diag"
name=$1; file=$2; scales=$3; draw=${4:-true}; zoom=${5:-40}; poses=${6:-null}
rm -f "$G/res/$name.txt"
printf '{"kind":"diag","file":"%s","scales":%s,"draw":%s,"zoom":%s,"poses":%s,"out":"%s"}' "$file" "$scales" "$draw" "$zoom" "$poses" "$O" > "$G/req/$name.json.tmp"
mv "$G/req/$name.json.tmp" "$G/req/$name.json"
for i in $(seq 1 ${WAIT:-240}); do [ -f "$G/res/$name.txt" ] && break; sleep 1; done
cat "$G/res/$name.txt" 2>/dev/null || echo "no result yet"
