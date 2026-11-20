#!/bin/sh
# 🚢 Round 3, machinery/sorption-wheel: builds the volumes source and the candidate, validates the candidate and, only when it is valid, writes the roots and splices the volumes into the catalog asset. Usage: sh r3-sorption-wheel-ship.sh
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
CATALOG="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/sorption-wheel/🔣️.json"
python3 r3-sorption-wheel-volumes.py
python3 r3-sorption-wheel-roots.py
"$BUN" r3-validate.ts --file "🗑️generated/r3-sorption-wheel/🔣️.json" --volumes
python3 r3-sorption-wheel-apply.py
python3 volumes-insert.py volumes/machinery/sorption-wheel.json "$CATALOG"
"$BUN" r3-validate.ts machinery/sorption-wheel
