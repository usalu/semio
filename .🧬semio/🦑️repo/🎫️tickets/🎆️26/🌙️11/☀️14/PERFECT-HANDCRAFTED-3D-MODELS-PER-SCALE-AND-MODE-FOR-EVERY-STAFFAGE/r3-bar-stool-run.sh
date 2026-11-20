#!/bin/sh
# 🔁 Round 3, furniture/bar-stool: builds the candidate, validates it, writes catalog and source when valid, validates again.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
python3 r3-bar-stool-build.py > /dev/null
$BUN r3-validate.ts --file "🗑️generated/r3-bar-stool/🔣️.json" | cut -c1-400 | tail -12
$BUN r3-validate.ts --file "🗑️generated/r3-bar-stool/🔣️.json" > /dev/null
python3 r3-bar-stool-apply.py
$BUN r3-validate.ts furniture/bar-stool | tail -1
