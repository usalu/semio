#!/bin/sh
# 🚢 Round 3, machinery/finned-heat-exchanger: build the candidate, validate it, check print suitability, then write roots and volumes into the catalog file and run the product check.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
C="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/finned-heat-exchanger/🔣️.json"
python3 r3-finned-heat-exchanger-build.py >/dev/null
V=$($BUN r3-validate.ts --file "🗑️generated/r3-finned-heat-exchanger/🔣️.json" || true)
echo "$V" | cut -c1-500 | tail -12
echo "$V" | grep -q "^VALID"
$BUN r3-finned-heat-exchanger-suit.ts "🗑️generated/r3-finned-heat-exchanger/🔣️.json" | cut -c1-500
python3 r3-finned-heat-exchanger-apply.py | sed 's/.*members:/roots:/'
python3 volumes-insert.py volumes/machinery/finned-heat-exchanger.json "$C" | tail -1
$BUN volumes-check.ts machinery/finned-heat-exchanger --product 2>&1 | grep -E "^(FAIL|PASS)" | cut -c1-900
