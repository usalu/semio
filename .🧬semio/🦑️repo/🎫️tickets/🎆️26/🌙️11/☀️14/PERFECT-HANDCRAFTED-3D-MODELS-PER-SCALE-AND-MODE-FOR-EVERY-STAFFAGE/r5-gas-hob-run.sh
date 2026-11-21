#!/bin/sh
# 🔁 Round 5 gas hob: rebuild volumes and candidate, write the catalog only when the candidate validates, then check.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
mkdir -p "🗑️generated/r5-gas-hob"
python3 r5-gas-hob-model.py
python3 r5-gas-hob-roots.py
$BUN r3-validate.ts --file "🗑️generated/r5-gas-hob/🔣️.json" | tail -3
$BUN r3-validate.ts --file "🗑️generated/r5-gas-hob/🔣️.json" > /dev/null && python3 r5-gas-hob-roots.py --apply | tail -1
$BUN r3-validate.ts appliances/gas-hob | tail -1
$BUN volumes-check.ts appliances/gas-hob --product 2>&1 | grep -E "^(FAIL|ok)" | sed 's/info:[^ ]*//g' | cut -c1-600 > "🗑️generated/r5-gas-hob/check.txt"
grep -E "^FAIL" "🗑️generated/r5-gas-hob/check.txt" || echo "all ok"
