#!/bin/bash
# Usage: run.sh <slug> [--splice]  : author, check (volume file), optionally splice, validate and inspect
set -e
T="$(cd "$(dirname "$0")/.." && pwd)"
cd "$T"
SLUG="$1"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
OUT="$T/🗑️generated/authoring-humans-a"
mkdir -p "$OUT"
(cd authoring-humans-a && python3 "a_${SLUG}.py")
$BUN volume-check.ts "$SLUG" --volume "volumes/humans/$SLUG.json" | grep -E "x \(|y \(|z \(|IoU|PASS|FAIL|outside|misses"
if [ "$2" = "--splice" ]; then
  FILE=$(ls -d /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*humans/$SLUG)/🔣️.json
  python3 volume-insert.py "volumes/humans/$SLUG.json" "$FILE"
  $BUN volume-check.ts "$SLUG" --json > "$OUT/$SLUG.check.json" || true
  $BUN authoring-humans-a/validate.ts "$SLUG" | tee "$OUT/$SLUG.validate.txt"
  $BUN authoring-humans-a/inspect.ts "$SLUG" ${3:+--poses $3} | tee "$OUT/$SLUG.poses.txt"
fi
