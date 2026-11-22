#!/bin/bash
# usage: preview.sh <asset-id> <scale> [view] [pose] — renders the three views (first-angle sheet) of the catalog asset to 🗑️generated/PW-13/<asset>/ and rasterises them with qlmanage (no rsvg-convert on this machine)
set -e
HERE=$(cd "$(dirname "$0")"; pwd); S="$HERE/../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage"; OUT="$HERE/../../🗑️generated/PW-13/$1"; mkdir -p "$OUT"
cd "$S"; repo tools exec bun -- 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset "$1" --scale "$2" --views elevation,front,plan --projection first --view "${3:-dark-line}" ${4:+--pose $4} --language en --no-png --out "$OUT" 2>&1 | grep -v "^\[" | tail -3
cd "$OUT"; for f in *-1-$2.svg; do qlmanage -t -s 2400 -o . "$f" >/dev/null 2>&1; done; ls "$OUT" | grep "1-$2.svg.png"
