#!/bin/sh
# Renders the review sheets of candidates one tool process at a time and tiles them into two contact sheets per asset in 🗑️generated/sheets: <slug>.3d.png (1:50 body stored, carried, played and the 1:200 body) and <slug>.2d.png (the 2D drawings of the four scales, authored ones only).
R=/Users/ueli/Documents/semio
K="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/USED-MUSICAL-INSTRUMENTS-STAFFAGE-COLLECTION"
T="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
BUN=$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)
O="$K/🗑️generated/sheets"
mkdir -p "$O"
cd "$T" || exit 1
path() { grep -o '[^ ]*\.png' | tail -1; }
for slug in "$@"; do
  C="$K/assets/$slug/🔣️.json"
  a=$("$BUN" r3-sheet.ts --file "$C" --scale 50 --pose stored 2>&1 | path)
  b=$("$BUN" r3-sheet.ts --file "$C" --scale 50 --pose carried 2>&1 | path)
  c=$("$BUN" r3-sheet.ts --file "$C" --scale 50 --pose played 2>&1 | path)
  d=$("$BUN" r3-sheet.ts --file "$C" --scale 200 2>&1 | path)
  "$BUN" r5-grid.ts "$O/$slug.3d.png" 2 2 "$a" "$b" "$c" "$d" > /dev/null 2>&1
  set --  "$@"
  twod=""
  for n in 50 100 200 500; do
    p=$("$BUN" r3-2d.ts --file "$C" --scale $n 2>&1 | path)
    [ -n "$p" ] && [ -f "$p" ] && twod="$twod $p"
  done
  "$BUN" r5-grid.ts "$O/$slug.2d.png" 2 2 $twod > /dev/null 2>&1
  echo "$slug: $(ls "$O" | grep -c "^$slug\.")"
  rm -f "$T/🗑️generated/r3-sheet/file/$slug".* "$T/🗑️generated/r3-2d/file/$slug".*
done
