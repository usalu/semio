#!/bin/sh
# 🌊️ Sweeps every person of today's human wave (ids = row files of the wave ticket) with `sweep-2.ts`, one process: `sweep-wave-3.sh <tag>` writes `🗑️generated/tv3/sweep-<tag>.txt` and prints the summary line of every person that has findings.
HERE="$(cd "$(dirname "$0")" && pwd)"; U="$(dirname "$HERE")"
K="$U/../STAFFAGE-HUMAN-COLLECTION-WAVE"
S="$U/../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
OUT="$U/🗑️generated/tv3"; mkdir -p "$OUT"
IDS=$(ls "$K/📥️manifest-rows" | sed 's/\.json$//')
cd "$S" && timeout 3000 "$BUN" "$HERE/sweep-2.ts" $IDS > "$OUT/sweep-$1.txt" 2>&1
echo "exit $?"
grep ": tolerance " "$OUT/sweep-$1.txt" | grep -v "sweep 0, consistency 0"
echo "green: $(grep -c 'sweep 0, consistency 0' "$OUT/sweep-$1.txt")"
