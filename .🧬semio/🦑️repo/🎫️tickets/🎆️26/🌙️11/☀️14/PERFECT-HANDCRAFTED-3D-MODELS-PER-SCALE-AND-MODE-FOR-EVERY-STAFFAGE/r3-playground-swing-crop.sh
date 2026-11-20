#!/bin/sh
# 🔍 Crops a region (x y w h in pixels) of a PNG and enlarges it to 1200 px wide: r3-playground-swing-crop.sh <in.png> <x> <y> <w> <h> <out.png>
set -e
cp "$1" "$6"
sips -c "$5" "$4" --cropOffset "$3" "$2" "$6" >/dev/null
sips --resampleWidth 1200 "$6" >/dev/null
echo "$6"
