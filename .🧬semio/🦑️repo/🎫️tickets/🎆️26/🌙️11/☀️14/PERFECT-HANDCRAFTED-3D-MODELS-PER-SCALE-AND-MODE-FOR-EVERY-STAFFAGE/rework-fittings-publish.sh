#!/bin/sh
# 📮 Publishes one volumes source of the fittings rework and proves the catalog file: sh rework-fittings-publish.sh <category>/<slug>
set -e
T=$(cd "$(dirname "$0")" && pwd)
R=${T%%/.🧬semio/*}
S="$R/🎓️teaching/🏛️architecture/🧍️staffage"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
case "${1%%/*}" in components) F=🧱️components ;; objects) F=🪑️objects ;; vehicles) F=🚜️vehicles ;; esac
cd "$T"
python3 volumes-insert.py "volumes/$1.json" "$S/🗂️catalog/$F/${1##*/}/🔣️.json"
"$BUN" r3-validate.ts "$1" --catalog-only --volumes 2>&1 | tail -3 | cut -c1-600
cd "$R"
"$BUN" "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "${1##*/}" 2>&1 | grep -E "✓|✗|rror" | head -5
