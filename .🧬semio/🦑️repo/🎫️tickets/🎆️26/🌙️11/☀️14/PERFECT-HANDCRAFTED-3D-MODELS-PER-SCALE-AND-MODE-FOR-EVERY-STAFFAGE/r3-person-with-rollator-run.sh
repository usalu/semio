#!/bin/sh
# 🛒 Rebuilds humans/person-with-rollator: volumes source, candidate document, then renders the sheets named as arguments ("50 body", "100 structure --cut", ...).
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
"$BUN" r3-person-with-rollator-build.ts ${ONLY:+--only $ONLY} > /dev/null
python3 r3-person-with-rollator-candidate.py > /dev/null
for spec in "$@"; do
  set -- $spec
  scale=$1; list=$2; shift 2
  "$BUN" r3-person-with-rollator-sheet.ts humans/person-with-rollator --file "🗑️generated/r3-person-with-rollator/🔣️.json" --scale "$scale" --list "$list" --no-labels "$@" 2>&1 | tail -3
done
