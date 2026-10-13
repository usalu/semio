#!/bin/bash
# 🧪️ Runs the repository-test SUBJECT phase (exhaustive, rust) for every semio subset owner, one owner per harness
# run, resumably: owners already recorded in the result table are skipped; stops when the time budget runs out.
# usage: 🧪️subject-sweep.sh <seconds>
cd /Users/ueli/Documents/semio || exit 2
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"
OUT="$T/🗑️generated/lane-c/subject-sweep.tsv"; touch "$OUT"
BUDGET="$1"; start=$(date +%s)
SUBSETS="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets"
for owner in "$SUBSETS"/*/; do
  owner="${owner%/}"
  cut -f2 "$OUT" | grep -qxF "$owner" && continue
  left=$(( BUDGET - ($(date +%s) - start) )); [ "$left" -lt 90 ] && { echo "budget exhausted"; break; }
  log="$T/🗑️generated/lane-c/subject-$(basename "$owner").log"
  bash "$T/🚦️cargo-slot.sh" lane-c -- bash "$T/wave0-feature-drift/⏱️bounded.sh" "$left" -- bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts subject exhaustive --owner "$owner" --implementation rust > "$log" 2>&1
  rc=$?
  [ "$rc" = 124 ] && { echo "deadline in $owner"; break; }
  printf "%s\t%s\t%s\n" "$rc" "$owner" "$(grep -E '^\[test\] level=' "$log" | tail -1)" >> "$OUT"
done
cut -f1,3 "$OUT" | sort | uniq -c | sort -rn | head -40
