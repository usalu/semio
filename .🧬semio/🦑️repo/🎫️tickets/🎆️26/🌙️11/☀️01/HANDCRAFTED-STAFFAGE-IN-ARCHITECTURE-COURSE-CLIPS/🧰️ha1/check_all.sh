#!/bin/bash
cd /c/git/semio
S="🎓️teaching/🏛️architecture/🧍️staffage"
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
OUT="$T/🗑️generated/ha1/check"
mkdir -p "$OUT"
for id in "$@"; do
  repo tools exec bun -- "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$id" > "$OUT/$id.txt" 2>&1
  echo "done $id" >> "$OUT/_progress.txt"
done
echo ALLDONE >> "$OUT/_progress.txt"
