#!/bin/sh
# ✂️ Crops a region of a PNG render for looking at details (macOS sips). Usage: sh r3-domestic-cat-crop.sh <in.png> <out.png> <x> <y> <width> <height>
sips -c "$6" "$5" --cropOffset "$4" "$3" "$1" --out "$2" >/dev/null
