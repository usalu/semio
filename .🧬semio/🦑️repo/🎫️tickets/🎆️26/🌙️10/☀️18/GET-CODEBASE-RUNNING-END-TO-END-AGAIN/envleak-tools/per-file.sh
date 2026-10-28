#!/bin/sh
# per-file.sh <mirror> <out-file> <rel-test-file>... : runs each test file in its own bun process inside the mirror, prints "pass fail file".
MIRROR=$1; OUT=$2; shift 2
HERE=$(cd "$(dirname "$0")/.." && pwd)
: > "$OUT"
for f in "$@"; do
  r=$(python3 -I "$HERE/mirror-tools/mirror.py" run "$MIRROR" --peers-ok -- bun test "./$f" 2>&1 | grep -E "^ *[0-9]+ (pass|fail)$" | tr '\n' ' ')
  echo "$r | $f" >> "$OUT"
done
