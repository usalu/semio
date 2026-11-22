#!/bin/sh
# 📤 Builds the candidate of one appliance, validates it with the measured volume rules and publishes only when it is VALID.
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
python3 rework-16a-$1-model.py >/dev/null || exit 1
OUT=$($BUN r3-validate.ts --file "🗑️generated/r3-$1/🔣️.json" --volumes 2>&1)
echo "$OUT" | cut -c1-700 | tail -${2:-12}
echo "$OUT" | tail -1 | grep -q '^VALID' || exit 1
python3 rework-16a-$1-model.py --publish | tail -1
$BUN r3-validate.ts appliances/$1 --catalog-only | tail -1
