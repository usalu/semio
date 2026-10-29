#!/bin/bash
# [DEBUG] build -> measure advance -> rebuild -> check -> sketch.  usage: cycle.sh <asset> [sketch specs...]
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE"; S=/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage; G="$T/🗑️generated/humans-a"; A=$1; shift
cd "$T/🧰️scripts/humans-a" && node build.mjs 2>&1 | grep -v "clamp\|^wrote"
cd /home/deck/semio
repo tools exec bun -- "$T/🧰️scripts/humans-a-probe.ts" "$S/🗂️catalog/👤️humans/🔣️.json" $A "$T/🧰️scripts/humans-a/advance-$A.json" 2>&1 | grep -v "^\[\|^$"
cd "$T/🧰️scripts/humans-a" && node build.mjs 2>&1 | grep -v "clamp\|^wrote"
cd /home/deck/semio
repo tools exec bun -- "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --file "$S/🗂️catalog/👤️humans/🔣️.json" --asset $A 2>&1 | grep -v "^\[tools\|^\[capacity"
if [ $# -gt 0 ]; then repo tools exec bun -- "$T/🧰️scripts/humans-a-sketch.ts" "$S/🗂️catalog/👤️humans/🔣️.json" $A "$G/$A.svg" "$@" 2>&1 | grep -v "^\[" ; rsvg-convert -w 2800 "$G/$A.svg" -o "$G/$A.png"; fi
