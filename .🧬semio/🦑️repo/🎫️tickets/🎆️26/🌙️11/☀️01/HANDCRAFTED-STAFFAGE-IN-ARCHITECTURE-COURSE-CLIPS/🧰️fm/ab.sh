#!/usr/bin/env bash
# Usage: ab.sh [ids]: A = exporter copy with the pre-change measure (generated/fm/orig), B = current tree; same catalog, back to back, one process each; prints CPU and the digest diff.
set -u
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/fm"
S="🎓️teaching/🏛️architecture/🧍️staffage"
for side in A B; do
  exporter="$S/🔨️modules/📤️export/🎭️cast/🟦️.ts"
  [ "$side" = A ] && exporter="$G/orig/🔨️modules/📤️export/🎭️cast/🟦️.ts"
  rm -rf "$G/ab-$side"
  for attempt in 1 2 3 4 5; do
    out=$(bun "$T/🧰️pf/bench.ts" "$exporter" "$G/manifest.json" "$G/ab-$side" "$S/🗂️catalog" "${1:-}" 2>&1 | tail -1)
    case "$out" in *"[cpu]"*) break ;; esac
    sleep 15
  done
  echo "$side: $out"
  PYTHONIOENCODING=utf-8 python3 "$T/🧰️fm/digests.py" "$G/ab-$side" > "$G/digests-ab-$side.txt"
done
wc -l "$G/digests-ab-A.txt" "$G/digests-ab-B.txt" | head -2
if diff -q "$G/digests-ab-A.txt" "$G/digests-ab-B.txt" > /dev/null; then echo IDENTICAL; else echo DIFFERENT; diff "$G/digests-ab-A.txt" "$G/digests-ab-B.txt" | awk '/^[<>]/{print $3}' | cut -d/ -f1 | sort | uniq -c; fi
