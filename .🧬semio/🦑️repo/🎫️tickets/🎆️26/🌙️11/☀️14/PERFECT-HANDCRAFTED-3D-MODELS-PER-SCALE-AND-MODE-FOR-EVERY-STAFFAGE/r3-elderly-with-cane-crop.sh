#!/bin/sh
# ✂️ Crops a region of a rendered sheet and enlarges it for a close look: crop.sh in.png out.png x y width height [zoom]
set -e
z=${7:-2}
sips -c "$6" "$5" --cropOffset "$4" "$3" "$1" --out "$2" >/dev/null
sips -z $(( $6 * z )) $(( $5 * z )) "$2" >/dev/null
echo "$2"
