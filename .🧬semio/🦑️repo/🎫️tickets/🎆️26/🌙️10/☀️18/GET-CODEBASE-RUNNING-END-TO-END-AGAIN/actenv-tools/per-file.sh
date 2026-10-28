#!/bin/sh
# per-file.sh <mirror> <out-file> <files-list>: runs each listed test file in its own bun process inside the mirror, appends "pass fail | file" per file (skips files already in out-file).
MIRROR=$1; OUT=$2; LIST=$3
HERE=$(cd "$(dirname "$0")/../mirror-tools" && pwd)
touch "$OUT"
while IFS= read -r f; do
  grep -qF "| $f" "$OUT" && continue
  r=$(python3 -I "$HERE/mirror.py" run "$MIRROR" --peers-ok -- bun test "./$f" 2>&1 | grep -E "^ *[0-9]+ (pass|fail)$" | tr '\n' ' ')
  echo "$r | $f" >> "$OUT"
done < "$LIST"
