#!/usr/bin/env bash
W="/c/git/semio"
T="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/r3-valid"
S="$W/🎓️teaching/🏛️architecture/🧍️staffage"
mkdir -p "$G"
cd "$S/📦️packages/🟦️typescript" && repo tools exec bun -- "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --samples 5 > "$G/catalog.log" 2>&1 < /dev/null
echo "catalog rc=$?" >> "$G/status.txt"
