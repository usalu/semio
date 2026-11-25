#!/bin/zsh
# ✂️ The drawing row (light-solid) of sheets, cut out and scaled to 1300 px for a quick look: crop.sh <sheet.png>… writes `<sheet>.row.png` beside each (macOS `sips`).
for f in "$@"; do
  w=$(sips -g pixelWidth "$f" | awk '/pixelWidth/{print $2}'); h=$(sips -g pixelHeight "$f" | awk '/pixelHeight/{print $2}')
  sips -c $(( h * 19 / 100 )) $w --cropOffset $(( h * 55 / 100 )) 0 "$f" --out "${f:r}.row.png" > /dev/null && sips --resampleWidth 1300 "${f:r}.row.png" > /dev/null && echo "${f:r}.row.png"
done
