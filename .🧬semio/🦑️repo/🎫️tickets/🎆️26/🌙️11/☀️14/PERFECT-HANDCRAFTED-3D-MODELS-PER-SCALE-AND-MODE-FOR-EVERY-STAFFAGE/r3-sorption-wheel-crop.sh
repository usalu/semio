#!/bin/sh
# 🔍 Crops a region (x y w h) out of a PNG and enlarges it by a factor, with sips: r3-sorption-wheel-crop.sh in.png out.png x y w h factor
set -e
cp "$1" "$2"
sips --cropOffset "$4" "$3" -c "$6" "$5" "$2" >/dev/null
sips -z $(( $6 * $7 )) $(( $5 * $7 )) "$2" >/dev/null
