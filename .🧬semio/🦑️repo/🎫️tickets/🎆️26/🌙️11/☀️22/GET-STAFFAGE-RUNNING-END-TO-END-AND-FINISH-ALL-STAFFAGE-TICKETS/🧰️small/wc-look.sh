#!/bin/sh
# 👓️ Renders the product 2D sheet (side, front, plan; line and solid view) of one document of the walking commuter at one scale and one pose or clip instant with the r3-2d tool of the 3D ticket and moves the PNG to `🗑️generated/small/wc/sheets/`: `wc-look.sh <asset document json> <name> <scale> pose <pose>` or `wc-look.sh <asset document json> <name> <scale> clip <clip> <t>`. Prints the path of the PNG.
HERE="$(cd "$(dirname "$0")" && pwd)"; U="$(dirname "$HERE")"
T14="$U/../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
FILE="$1"; NAME="$2"; SCALE="$3"; KIND="$4"; shift 4
OUT="$U/🗑️generated/small/wc/sheets"; mkdir -p "$OUT"
WORK="$OUT/work/walking-commuter"; mkdir -p "$WORK"; cp "$FILE" "$WORK/🔣️.json"
cd "$T14" || exit 1
if [ "$KIND" = pose ]; then PNG=$(timeout 300 "$BUN" r3-2d.ts --file "$WORK/🔣️.json" --scale "$SCALE" --pose "$1" | tail -1); else PNG=$(timeout 300 "$BUN" r3-2d.ts --file "$WORK/🔣️.json" --scale "$SCALE" --clip "$1" --t "$2" | tail -1); fi
rm -rf "$OUT/work"
[ -f "$PNG" ] || { echo "no sheet: $PNG"; exit 1; }
mv "$PNG" "$OUT/$NAME-$SCALE-$KIND-$1${2:+-$2}.png" && echo "$OUT/$NAME-$SCALE-$KIND-$1${2:+-$2}.png"
