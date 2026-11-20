#!/bin/sh
# 🔍 Round 4 group G11: crops a rendered sheet to a pixel box (x0 y0 x1 y1) and doubles it, to look closely at seams and joints; writes <png stem>.crop.png next to it (macOS sips).
set -e
out="${1%.png}.crop.png"
w=$(( $4 - $2 )); h=$(( $5 - $3 ))
sips -c "$h" "$w" --cropOffset "$3" "$2" "$1" --out "$out" >/dev/null
sips -z $(( h * 2 )) $(( w * 2 )) "$out" >/dev/null
echo "$out"
