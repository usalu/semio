#!/bin/sh
# 🏃️ PW-12 check runner: the staffage check CLI and the volume probe for the given assets, outputs into the generated folder.
S="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage"
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-PLANT-COLLECTION-WAVE"
G="$T/🗑️generated/PW-12"
SAMPLES="${SAMPLES:-2}"
cd "$S" || exit 1
for a in "$@"; do
  repo tools exec bun -- 🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset "$a" --samples "$SAMPLES" > "$G/check-$a.txt" 2>&1
  echo "== check $a"; grep -v "^\[" "$G/check-$a.txt" | cut -c1-600
  PW12_MEASURE=1 repo tools exec bun -- "$T/🧰️scripts/PW-12/vcheck.ts" "$a" all > "$G/v-$a.txt" 2>"$G/v-$a.err"
  echo "== volumes $a"; grep -v "ok " "$G/v-$a.txt" | cut -c1-700
done
