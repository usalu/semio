#!/bin/zsh
# 🖼️ Renders third-angle three-view sheets (light solid) of an asset file and rasterises them: sheet.sh <slug|file.json> <out-name> "<scales>" [preview args, e.g. --pose id | --clip id --time s | --view thermal]
source "${0:A:h}/env.sh"
if [[ "$1" == *.json ]]; then f="${1:A}"; else f="$CAT/$1/🔣️.json"; fi
o="$GEN/sheets/$2"; mkdir -p "$o"; scales=(${=3}); shift 3
id=$(python3 -c "import json,sys;print(json.load(open(sys.argv[1]))['asset']['id'])" "$f")
for s in $scales; do
  (cd "$S" && $B 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --file "$f" --asset "$id" --scale $s --views elevation,front,plan --projection third --view light-solid --no-png --out "$o" "$@" > "$GEN/[DEBUG]-preview.log" 2>&1) || { echo "[DEBUG] preview failed $s"; tail -3 "$GEN/[DEBUG]-preview.log"; }
done
(cd "$T/🧰️scripts/HW-01" && $B png.ts "$o" ${DPI:-220} > /dev/null) && ls "$o"
