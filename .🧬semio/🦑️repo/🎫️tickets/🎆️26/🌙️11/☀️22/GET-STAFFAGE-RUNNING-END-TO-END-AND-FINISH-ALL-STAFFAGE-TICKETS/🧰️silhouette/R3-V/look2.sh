#!/bin/zsh
# 👀️ Renders the product drawings of a vehicle document at a scale (side, front, plan; line and solid view) without the dependency image: svg.ts writes the SVGs, sheet.py joins them, QuickLook rasterises the sheet; prints the PNG: look2.sh <asset document json> <id> <scale> <tag> [modes]
source "${0:A:h}/env.sh"
OUT="$GEN/svg/$2/$4"; mkdir -p "$OUT"
(cd "$S/📦️packages/🟦️typescript" && timeout 300 "$B" "$W/svg.ts" "$1" "$3" "$OUT" ${5:-light-line,light-solid} > /dev/null) || { echo "no drawings"; exit 1 }
python3 "$W/sheet.py" "$OUT" "$3" "$OUT/sheet-$3.svg" ${5:-light-line,light-solid} > /dev/null
(cd "$OUT" && qlmanage -t -s ${SIZE:-1800} -o . "sheet-$3.svg" > /dev/null 2>&1)
echo "$OUT/sheet-$3.svg.png"
