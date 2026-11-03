#!/bin/bash
# usage: rebuild.sh asset-id castgroup
cd /c/git/semio
S="🎓️teaching/🏛️architecture/🧍️staffage"
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/a6"
F='d3-format\|\^\|package.json\|347 |\|^\[tools\|^\[capacity\|^\[repo'
repo tools exec bun -- "$T/🧰️a6/build.mjs" "$S/🗂️catalog" "$1" 2>&1 | grep -v "$F"
repo tools exec bun -- "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$1" 2>&1 | grep -v "$F" | cut -c1-600 | head -${3:-40}
"$T/🧰️a6/cast.sh" "$2" > /dev/null
