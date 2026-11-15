#!/bin/sh
# crop.sh <png> <out> <y> <x> <h> <w>
sips -c $5 $6 --cropOffset $3 $4 "$1" --out "$2" >/dev/null
