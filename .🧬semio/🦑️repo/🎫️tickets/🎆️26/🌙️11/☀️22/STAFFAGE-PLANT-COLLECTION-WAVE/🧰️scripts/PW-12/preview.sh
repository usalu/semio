#!/bin/sh
# 🖼️ PW-12 previews: the default pose of each given asset at 1:50, 1:100, 1:200 and 1:500 (side elevation, light solid, print size)
# and a third-angle sheet of plan, front and side at every scale, written to the generated folder.
S="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage"
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-PLANT-COLLECTION-WAVE/🗑️generated/PW-12"
VIEW="${VIEW:-light-solid}"
cd "$S" || exit 1
for a in "$@"; do
  for s in 50 100 200 500; do
    repo tools exec bun -- 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset "$a" --scale "$s" --view "$VIEW" --views elevation,front,plan --projection third --language en --dpi 300 --out "$G/preview/$a/$s" > "$G/preview-$a-$s.log" 2>&1
    echo "== $a 1:$s $(tail -1 "$G/preview-$a-$s.log")"
  done
done
