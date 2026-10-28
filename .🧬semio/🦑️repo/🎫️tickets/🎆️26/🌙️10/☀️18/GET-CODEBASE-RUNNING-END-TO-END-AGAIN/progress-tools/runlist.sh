#!/bin/sh
# usage: runlist.sh <mirror-name> <out-file> <list-file> <from> <to>   -- runs test dirs from..to (1-based) of the list
C="$(cd "$(dirname "$0")/.." && pwd)"; NAME="$1"; OUT="$2"; LIST="$3"; FROM="$4"; TO="$5"
mkdir -p "$C/🗑️generated/progress/art"
sed -n "${FROM},${TO}p" "$LIST" | while IFS= read -r d; do
  echo "### $d" >> "$OUT"
  SEMIO_TEST_ARTIFACT_DIR="$C/🗑️generated/progress/art" python3 -I "$C/mirror-tools/mirror.py" run "$NAME" --cwd "$d" -- bun test ./🟦️.ts 2>&1 | grep -E '^\((pass|fail)\)|^ *[0-9]+ (pass|fail)$' | sed 's/ \[[0-9.]*m\?s\]$//' >> "$OUT"
done
