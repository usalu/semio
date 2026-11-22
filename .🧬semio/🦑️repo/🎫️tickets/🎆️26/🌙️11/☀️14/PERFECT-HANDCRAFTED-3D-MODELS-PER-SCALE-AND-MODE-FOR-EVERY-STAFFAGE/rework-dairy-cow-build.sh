#!/bin/sh
# 🧰 Builds the dairy cow candidate document from the frozen catalog copy: rig, body, structure chisels in order.
set -e
cd "$(dirname "$0")"
G="🗑️generated/rework-dairy-cow"
OUT="🗑️generated/r3-dairy-cow/🔣️.json"
mkdir -p "🗑️generated/r3-dairy-cow"
python3 rework-dairy-cow-rig.py "$G/catalog-before.json" "$OUT" > "$G/rig.log"
python3 rework-dairy-cow-body.py "$OUT"
if [ -f rework-dairy-cow-structure.py ]; then python3 rework-dairy-cow-structure.py "$OUT"; fi
python3 rework-dairy-cow-girdle.py "$OUT"
python3 rework-dairy-cow-decisions.py "$OUT"
