#!/bin/sh
# Round 3, appliances/decentralised-hrv-fan: rebuild the volumes source and the candidate, validate, write the roots and splice the volumes.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
CATALOG="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/decentralised-hrv-fan/🔣️.json"
python3 r3-decentralised-hrv-fan-volumes.py
python3 r3-decentralised-hrv-fan-roots.py
"$BUN" r3-validate.ts --file "🗑️generated/r3-decentralised-hrv-fan/🔣️.json" --volumes
python3 r3-decentralised-hrv-fan-apply.py
python3 volumes-insert.py volumes/appliances/decentralised-hrv-fan.json "$CATALOG"
"$BUN" r3-validate.ts appliances/decentralised-hrv-fan --catalog-only
