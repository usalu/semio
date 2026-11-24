#!/bin/zsh
# 🔬️ Product drawings of a document as SVG (svg.ts) and one magnified part of a view: zoom.sh <asset json> <tag> <scale> <mode> <view> <x0> <y0> <x1> <y1> [width]   (fractions of the drawing; prints the PNG path)
source "${0:A:h}/env.sh"
D="$GEN/svg/$2"; mkdir -p "$D"
cd "$S/📦️packages/🟦️typescript" || exit 1
[[ -f "$D/$3-$4-$5.svg" ]] || timeout 900 "$B" "$W/svg.ts" "$1" "$3" "$D" "$4" > /dev/null || exit 1
timeout 300 "$B" "$W/zoom.ts" "$D/$3-$4-$5.svg" "$D/$3-$4-$5-$6-$7.png" "$6" "$7" "$8" "$9" "${10:-1400}" | cut -d' ' -f1
