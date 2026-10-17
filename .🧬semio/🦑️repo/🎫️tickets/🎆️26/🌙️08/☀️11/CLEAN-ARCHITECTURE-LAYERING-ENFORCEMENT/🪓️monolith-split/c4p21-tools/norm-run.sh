#!/bin/bash
# usage: norm-run.sh <phase>
cd /Users/ueli/Documents/semio || exit 1
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G="$F/🗑️generated"
N="✏️s/🔌️plugins/📕️norm"
for d in "$N"/🗿️artifacts/*/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/*mutate-*-1; do
  k=$(basename "$d")
  python3 "$F/c4p21-tools/pyrun.py" "$PWD/$d" "$PWD/$d/🐍️.py" "$PWD/$G/c4p21-py-norm-$k-$1.jsonl" "$PWD/$N/🔮️oracles/🏃️execution" > "$G/c4p21-py-norm-$k-$1.log" 2>&1
  echo "$k $(tail -1 "$G/c4p21-py-norm-$k-$1.log")"
done
