#!/bin/sh
# 🔁 Round 3, machinery/diesel-generator: build the candidate, validate it, write the roots and splice the volumes, validate the catalog (stops at the first failure).
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
CAT="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/diesel-generator/🔣️.json"
python3 r3-diesel-generator-build.py > /dev/null
$BUN r3-diesel-generator-suit.ts "🗑️generated/r3-diesel-generator/🔣️.json" | grep -v "suitable=true" || true
$BUN r3-validate.ts --file "🗑️generated/r3-diesel-generator/🔣️.json"
python3 r3-diesel-generator-apply.py
python3 volumes-insert.py volumes/machinery/diesel-generator.json "$CAT"
$BUN r3-validate.ts machinery/diesel-generator --catalog-only
