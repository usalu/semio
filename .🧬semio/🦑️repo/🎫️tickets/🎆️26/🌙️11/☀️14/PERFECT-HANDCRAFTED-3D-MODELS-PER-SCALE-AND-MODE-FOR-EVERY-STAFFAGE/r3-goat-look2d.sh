#!/bin/sh
# 🔍️ Round 3 goat: draws the candidate document's 2D panels of a scale as SVG and enlarges them with Quick Look for a close look. Usage: sh r3-goat-look2d.sh <scale> [pose]
set -e
T="$(cd "$(dirname "$0")" && pwd)"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
cd "$T"
if [ -n "$2" ]; then "$BUN" r3-2d.ts animals/goat --scale "$1" --pose "$2" --file "🗑️generated/r3-goat/🔣️.json" --svg >/dev/null; else "$BUN" r3-2d.ts animals/goat --scale "$1" --file "🗑️generated/r3-goat/🔣️.json" --svg >/dev/null; fi
OUT="$T/🗑️generated/r3-goat/ql"
mkdir -p "$OUT"
cd "$T/🗑️generated/r3-2d/file"
for f in r3-goat.s$1*.svg; do qlmanage -t -s 2400 -o "$OUT" "$f" >/dev/null 2>&1; done
ls "$OUT" | grep "s$1"
