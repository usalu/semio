#!/bin/sh
# 🚢 Round 3, machinery/evaporative-humidifier: build the candidate, validate it, write the roots and splice the volumes, then check.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A=machinery/evaporative-humidifier
C="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/evaporative-humidifier/🔣️.json"
python3 r3-evaporative-humidifier-build.py > /dev/null
$BUN r3-validate.ts --file "🗑️generated/r3-evaporative-humidifier/🔣️.json" > "🗑️generated/r3-evaporative-humidifier/validate.txt" || { tail -20 "🗑️generated/r3-evaporative-humidifier/validate.txt"; exit 1; }
tail -1 "🗑️generated/r3-evaporative-humidifier/validate.txt"
$BUN r3-plate-heat-exchanger-suit.ts "🗑️generated/r3-evaporative-humidifier/🔣️.json"
python3 r3-evaporative-humidifier-apply.py | sed -e 's/.*members:/roots written:/'
python3 volumes-insert.py volumes/$A.json "$C" | tail -1
$BUN volumes-check.ts $A --product 2>&1 | grep -v "^\[check\]" | grep "FAIL\|PASS\|!" | sed -e 's/info:[^ ]*//g' | cut -c1-700
