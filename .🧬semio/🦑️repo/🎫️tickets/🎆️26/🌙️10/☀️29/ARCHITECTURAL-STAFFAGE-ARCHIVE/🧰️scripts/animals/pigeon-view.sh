#!/usr/bin/env bash
# [scratch] preview: pigeon-view.sh name [preview args...] renders $OUT/pigeon/name.png
A="$(cd "$(dirname "$0")" && pwd)"; OUT="$A/../../🗑️generated/animals"; n="$1"; shift
cd /home/deck/semio && repo tools exec bun -- "$A/preview.ts" --file "$OUT/pigeon.json" --asset pigeon --out "$OUT/pigeon/$n.svg" "$@" 2>&1 | grep -v "^\[" ; rsvg-convert -z 1.4 "$OUT/pigeon/$n.svg" -o "$OUT/pigeon/$n.png"
