#!/bin/sh
# 👀 Round 3 herring gull: roots into the trial document, model of the given scales, trial document with volumes, sheets of the given list (default body) at each scale. Usage: sh r3-herring-gull-look.sh "50 100" [list] [extra sheet args]
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
python3 r3-herring-gull-apply.py > /dev/null
for s in $1; do "$BUN" r3-herring-gull-model.ts $s; done
python3 r3-herring-gull-full.py > /dev/null
L=${2:-body}
shift 2 2>/dev/null || shift $#
for s in ${SHEETS:-50}; do "$BUN" r3-herring-gull-sheet.ts animals/herring-gull --scale $s --list $L "$@" 2>&1 | grep -v "^/" || true; done
