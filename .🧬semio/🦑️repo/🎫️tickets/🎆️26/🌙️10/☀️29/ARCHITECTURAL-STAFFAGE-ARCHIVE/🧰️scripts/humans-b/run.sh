#!/bin/bash
# usage: run.sh <asset-script-name> <asset-id> [check|preview ...]
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE"
S="/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage"
cd /home/deck/semio
F="$T/🧰️scripts/humans-b/out/$1.json"
if [ "$2" = "check" ]; then repo tools exec bun -- "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --file "$F" 2>&1 | grep -v '^\[' ; exit; fi
OUT="$T/🗑️generated/humans-b/$1-$3"
repo tools exec bun -- "$T/🧰️scripts/humans-b-preview.ts" --file "$F" --asset "$1" --out "$OUT.svg" --scale "$4" --views "$5" ${6:+--queries "$6"} 2>&1 | grep -v '^\[' 
rsvg-convert -z ${7:-4} "$OUT.svg" -o "$OUT.png" && echo "$OUT.png"
