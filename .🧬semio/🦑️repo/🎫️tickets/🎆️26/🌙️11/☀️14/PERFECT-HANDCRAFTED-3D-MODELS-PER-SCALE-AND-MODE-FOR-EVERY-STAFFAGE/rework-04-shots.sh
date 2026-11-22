#!/bin/sh
# 📸 Author 04: renders close-ups of a candidate with rework-04-sheet.ts. Usage: rework-04-shots.sh <category/slug> <scale> <list> <tag> <view> [box] [extra flags]
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A="$1"; S="$2"; LIST="$3"; TAG="$4"; VIEW="$5"; BOX="$6"; shift 6 2>/dev/null
F="🗑️generated/rework-04/${A#*/}/🔣️.json"
if [ -n "$BOX" ] && [ "$BOX" != "-" ]; then set -- --box "$BOX" "$@"; fi
if [ "$VIEW" != "all" ]; then set -- --only "$VIEW" "$@"; fi
"$BUN" rework-04-sheet.ts "$A" --scale "$S" --list "$LIST" --file "$F" --tag "$TAG" "$@" 2>&1 | tail -3
