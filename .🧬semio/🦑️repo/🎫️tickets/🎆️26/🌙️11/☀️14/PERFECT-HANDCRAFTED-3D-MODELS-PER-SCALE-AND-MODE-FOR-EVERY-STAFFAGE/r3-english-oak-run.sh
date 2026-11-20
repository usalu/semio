#!/bin/sh
# 🔁 Round 3 English oak: regenerate, validate the candidate, splice roots and volumes into the catalog. Usage: sh r3-english-oak-run.sh
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/english-oak/🔣️.json"
$BUN r3-english-oak-model.ts | grep "1:"
$BUN r3-validate.ts --file "🗑️generated/r3-english-oak/🔣️.json" > "🗑️generated/r3-english-oak/validate.txt" || { cut -c1-300 "🗑️generated/r3-english-oak/validate.txt" | tail -8; exit 1; }
tail -1 "🗑️generated/r3-english-oak/validate.txt"
python3 r3-english-oak-splice.py "🗑️generated/r3-english-oak/roots.json" "$A"
python3 volumes-insert.py volumes/vegetation/english-oak.json "$A"
