#!/bin/sh
# 📦 Publishes the volumes source of one component into its catalog file and proves it: cheap validation, full measured proof, product check CLI. One tool process at a time.
# Usage: sh rework-walls-publish.sh <slug>
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
S="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage"
python3 volumes-insert.py "volumes/components/$1.json" "$S/🗂️catalog/🧱️components/$1/🔣️.json" | tail -2
"$BUN" r3-validate.ts "components/$1" --catalog-only 2>&1 | tail -4 | cut -c1-400
"$BUN" r3-validate.ts "components/$1" --catalog-only --volumes 2>&1 | tail -4 | cut -c1-400
"$BUN" "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$1" 2>&1 | grep -E "^[✓✗]|rror|✗" | cut -c1-400
