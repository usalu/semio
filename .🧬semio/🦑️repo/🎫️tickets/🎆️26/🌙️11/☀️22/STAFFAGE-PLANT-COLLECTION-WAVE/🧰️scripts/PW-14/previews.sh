#!/bin/sh
# 🖼️ Renders the three-view sheets (light solid) of one asset at every authored scale with sharp into the PW-14 generated folder.
# usage: sh previews.sh <asset-id> <scales...>
HERE=$(cd "$(dirname "$0")" && pwd)
ASSET="$1"; shift
FILE="$HERE/../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/$ASSET/🔣️.json"
OUT="$HERE/../../🗑️generated/PW-14/$ASSET"
mkdir -p "$OUT"
for S in "$@"; do
  repo tools exec bun -- "$HERE/preview.ts" --file "$FILE" --scale "$S" --views elevation,front,plan --projection third --view light-solid --dpi 400 --out "$OUT" 2>&1 | grep -v "capacity\|\[repo\]\|\[tools\]"
done
