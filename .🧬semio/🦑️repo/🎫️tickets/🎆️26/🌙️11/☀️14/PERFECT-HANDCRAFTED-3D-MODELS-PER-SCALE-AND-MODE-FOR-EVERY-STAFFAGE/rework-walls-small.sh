#!/bin/sh
# 🔎 Copies the sheets and 2D drawings of one slug at half size into 🗑️generated/rework-walls/look for reading.
cd "$(dirname "$0")" || exit 1
OUT="🗑️generated/rework-walls/look"
mkdir -p "$OUT"
for FILE in 🗑️generated/r3-sheet/components/"$1".s*.png 🗑️generated/r3-2d/components/"$1".s*.png; do
  case "$FILE" in *r3-2d*) NAME="2d-$(basename "$FILE")" ;; *) NAME="$(basename "$FILE")" ;; esac
  sips -Z 1200 "$FILE" --out "$OUT/$NAME" >/dev/null 2>&1
done
ls "$OUT" | grep "$1"
