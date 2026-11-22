#!/bin/sh
# 👀 Finisher A previews: the product's own 2D drawings (side, front, plan; line and solid) of one plant at 1:500, 1:200, 1:100 and 1:50 on one contact sheet, and the six shaded 3D views at 1:100; one tool process at a time; PNGs in 🗑️generated/finish-a/look/.
# usage: look.sh <id> [asset document json]   (default: the catalog file)
HERE="$(cd "$(dirname "$0")" && pwd)"
K="$HERE/../.."
P="$K/../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
S="$K/../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage"
G="$K/🗑️generated/finish-a/look"
BUN="$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)"
ID="$1"
FILE="${2:-$S/🗂️catalog/🌳️vegetation/$ID/🔣️.json}"
T="$G/tmp-$ID/$ID"
mkdir -p "$T"
cp "$FILE" "$T/🔣️.json"
cd "$P" || exit 1
for s in 500 200 100 50; do "$BUN" r3-2d.ts --file "$T/🔣️.json" --scale "$s" 2>&1 | grep -i -e error -e "NOT DRAWN"; done
"$BUN" r3-sheet.ts --file "$T/🔣️.json" --scale 100 --no-labels 2>&1 | grep -i -e error -e "NOT DRAWN"
"$BUN" "$HERE/compose.ts" "$G/$ID.2d.png" 2800 "🗑️generated/r3-2d/file/$ID.s500.png" "🗑️generated/r3-2d/file/$ID.s200.png" "🗑️generated/r3-2d/file/$ID.s100.png" "🗑️generated/r3-2d/file/$ID.s50.png"
mv "🗑️generated/r3-sheet/file/$ID.s100.body.no-labels.png" "$G/$ID.3d.png"
rm -f "🗑️generated/r3-2d/file/$ID.s500.png" "🗑️generated/r3-2d/file/$ID.s200.png" "🗑️generated/r3-2d/file/$ID.s100.png" "🗑️generated/r3-2d/file/$ID.s50.png"
rm -rf "$G/tmp-$ID"
