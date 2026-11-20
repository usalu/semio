#!/bin/sh
# 🔭 Rebuilds the candidate document of humans/seated-office-worker and renders the sheets named as arguments: "<scale> <list> [extra flags]" per quoted argument.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
"$BUN" r3-seated-office-worker-build.ts > /dev/null
for spec in "$@"; do
  set -- $spec
  scale=$1; list=$2; shift 2
  "$BUN" r3-seated-office-worker-sheet.ts humans/seated-office-worker --file "🗑️generated/r3-seated-office-worker/🔣️.json" --scale "$scale" --list "$list" "$@" 2>&1 | tail -1
done
