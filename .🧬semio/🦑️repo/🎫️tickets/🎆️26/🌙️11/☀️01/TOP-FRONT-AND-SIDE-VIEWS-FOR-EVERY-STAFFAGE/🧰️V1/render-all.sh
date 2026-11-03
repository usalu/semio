#!/bin/sh
cd /c/git/semio || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE"
G="$T/🗑️generated/V1"
r() {
  mkdir -p "$G/$1"
  repo tools exec bun -- "$T/🧰️V1/render.ts" "$1" "$2" "$3" "$G/$1" "$4" < /dev/null >> "$G/render-$1.log" 2>&1
}
r agave 500,200 summer-turgid 350
r agave 100,50 summer-turgid,drought-infolded,frost-sag,clip:wind-sway@1.5 500
r apple-tree 500,200,100,50 summer-fruit-set 350
r apple-tree 100 gale-windfall,clip:breeze-sway@1.5,clip:gust@1 350
r english-oak 500,200,100,50 calm 350
r english-oak 100 storm,clip:wind-sway@1.5 350
r bamboo-clump 500,200,100,50 summer-calm 350
r bamboo-clump 100 storm-bent,clip:wind-sway@1.5 400
r date-palm 500,200,100,50 calm 350
r date-palm 100 storm,clip:wind-sway@1.5 350
r hornbeam-hedge 500,200,100,50 calm 400
r hornbeam-hedge 100 snow,clip:wind-sway@1.5 400
r hosta-clump 200,100,50 summer-full 600
r hosta-clump 50 autumn-collapse,clip:wind-flutter@1.5 600
r ivy-on-trellis 500,200,100,50 summer-calm 400
r ivy-on-trellis 100 storm,clip:wind-sway@1.5 400
echo done > "$G/render-all.done"
