#!/bin/sh
# 🚢 Round 3, machinery/expansion-valve: build the candidate, validate it, write the roots and splice the volumes. Usage: sh r3-expansion-valve-ship.sh
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
C="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/expansion-valve/🔣️.json"
python3 r3-expansion-valve-build.py --source
"$BUN" r3-validate.ts --file "🗑️generated/r3-expansion-valve/🔣️.json" | tail -4
python3 r3-expansion-valve-apply.py
python3 volumes-insert.py volumes/machinery/expansion-valve.json "$C"
"$BUN" r3-validate.ts machinery/expansion-valve | tail -2
