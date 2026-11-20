#!/bin/sh
# ✂️ Crops a region (x y w h) out of a PNG with sips: r3-concrete-mixer-truck-crop.sh in.png out.png x y w h
sips --cropOffset "$4" "$3" -c "$6" "$5" "$1" --out "$2" >/dev/null
