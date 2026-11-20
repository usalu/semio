#!/bin/sh
# 🔁 Round 3 gas hob: rebuild volumes and candidate, write the catalog only when the candidate validates, then check.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
mkdir -p "🗑️generated/r3-gas-hob"
python3 r3-gas-hob-model.py
python3 r3-gas-hob-roots.py
$BUN r3-validate.ts --file "🗑️generated/r3-gas-hob/🔣️.json" | tail -3
$BUN r3-validate.ts --file "🗑️generated/r3-gas-hob/🔣️.json" > /dev/null && python3 r3-gas-hob-roots.py --apply | tail -1
$BUN r3-validate.ts appliances/gas-hob | tail -1
$BUN volumes-check.ts appliances/gas-hob --product 2>&1 | grep -E "^(FAIL|ok)" | sed 's/info:[^ ]*//g' | cut -c1-600 > "🗑️generated/r3-gas-hob/check.txt"
grep -E "^FAIL" "🗑️generated/r3-gas-hob/check.txt" || echo "all ok"
