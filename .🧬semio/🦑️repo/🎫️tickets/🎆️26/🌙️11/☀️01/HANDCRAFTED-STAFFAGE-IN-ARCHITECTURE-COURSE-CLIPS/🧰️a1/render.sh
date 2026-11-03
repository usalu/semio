#!/bin/bash
# usage: render.sh asset pose view scale outdir [clip-time-not-supported]
cd /c/git/semio
S="🎓️teaching/🏛️architecture/🧍️staffage"
repo tools exec bun -- "$S/🔨️modules/📤️export/🎬️cli/🟦️.ts" --asset "$1" --pose "$2" --view "$3" --scale "$4" --format svg --out "$5" --lang en 2>&1 | grep -v "^\[" | tail -2
