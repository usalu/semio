#!/bin/bash
# usage: cast.sh asset pose|clip names scales views [env] -> PNG sheets in generated/g1/<asset>/
cd /c/git/semio
S="🎓️teaching/🏛️architecture/🧍️staffage"
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/g1/$1"
mkdir -p "$G"
repo tools exec bun -- "$T/🧰️g1/casts.mjs" "$G/casts.json" "$1" "$2" "$3" "$4" "$5" "${6:-office-neutral}" >/dev/null 2>&1
ids=$(python -c "import json,sys;print(','.join(json.load(open(sys.argv[1]))['casts']))" "$G/casts.json")
rm -rf "$G/cast"
repo tools exec bun -- "$S/🔨️modules/📤️export/🎭️cast/🟦️.ts" --manifest "$G/casts.json" --ids "$ids" --out "$G/cast" --lang en 2>&1 | grep -iv "^\[" | tail -3
