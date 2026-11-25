#!/bin/sh
# 🧪️ Waits for the r47 browser build to leave the compiler, then runs the complete mounted crate in one run and records every result.
cd "$(dirname "$0")" || exit 1
G="$PWD/🗑️generated"
while pgrep -f "draw-e2e-2026-11-23.sh r4[7]" >/dev/null; do sleep 15; done
sh ./draw-native-tests.sh full-mounted semio-s-artifact-draw-drawing-scene-mounted -- --nocapture --test-threads=1 > /dev/null 2>&1
{
  echo "== complete mounted run $(date +%T) =="
  grep -a "^test \|^test result" "$G/native-full-mounted.txt" | cut -c1-200
  grep -a "panicked at" -A1 "$G/native-full-mounted.txt" | grep -av "controlled\|^--" | cut -c1-300
  echo FINISHED
} >> "$G/mounted-individual.txt"
