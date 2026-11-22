#!/bin/sh
# PW-16: generate one asset and probe it. usage: run.sh <generator.py> <asset-id> [measure.ts flags...]
set -e
HERE="$(cd "$(dirname "$0")" && pwd)"
S="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage"
gen="$1"; id="$2"; shift 2
cd "$HERE" && python3 "$gen"
cd "$S" && repo tools exec bun -- "$HERE/measure.ts" "🗂️catalog/🌳️vegetation/$id/🔣️.json" "$@" 2>&1 | grep -v "^\[capacity\]\|^\[repo\]\|^\[tools\]"
