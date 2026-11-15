#!/bin/bash
# usage: reset-patch.sh <draft module> <base module>: restores the replica files from base and re-applies patch-replica.py
cd "$(dirname "$0")"; D="$1"; B="$2"
for f in "🔁️replica/🟦️.ts" "🔁️replica/⌨️command/🟦️.ts" "🔁️replica/📖️texts/🔣️.json" "🔁️replica/🧬️schema/🔣️.json"; do cp "$B/$f" "$D/$f"; done
python3 patch-replica.py "$D"
