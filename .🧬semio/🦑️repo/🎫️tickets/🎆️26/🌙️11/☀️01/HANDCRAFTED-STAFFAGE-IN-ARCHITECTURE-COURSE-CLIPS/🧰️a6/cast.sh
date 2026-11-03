#!/bin/bash
cd /c/git/semio
S="🎓️teaching/🏛️architecture/🧍️staffage"
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/a6"
repo tools exec bun -- "$T/🧰️a6/casts.mjs" "$G/casts.json" "$1" 2>&1 | grep casts
ids=$(python -c "import json,sys;print(','.join(json.load(open(sys.argv[1]))['casts']))" "$G/casts.json")
rm -rf "$G/cast"
repo tools exec bun -- "$S/🔨️modules/📤️export/🎭️cast/🟦️.ts" --manifest "$G/casts.json" --ids "$ids" --out "$G/cast" --lang en 2>&1 | grep -v "^\[\|d3-format\|\^\|package.json\|347 |" | tail -8
ls "$G/cast" | wc -l
