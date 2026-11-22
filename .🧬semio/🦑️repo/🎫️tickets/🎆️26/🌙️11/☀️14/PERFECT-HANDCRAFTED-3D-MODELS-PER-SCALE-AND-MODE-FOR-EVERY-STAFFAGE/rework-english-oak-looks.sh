#!/bin/sh
# 🔎 English oak author: renders the poses, clips, lists and 2D drawings of the drawn candidate and keeps them reduced as look-<name>.png under 🗑️generated/rework-english-oak/english-oak.
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
G="🗑️generated/rework-english-oak/english-oak"
shot() { name=$1; shift; out=$("$BUN" "$@" 2>&1 | tail -1); if [ -f "$out" ]; then sips -Z 1500 "$out" --out "$G/look-$name.png" >/dev/null 2>&1; rm -f "$out"; else echo "FAILED $name $out"; fi; }
S="r3-sheet.ts --file $G/🔣️.json --no-labels"
shot p1 $S --scale 50 --pose windy --only right
shot p2 $S --scale 50 --pose storm --only right
shot p3 $S --scale 50 --pose wet-snow --only right
shot p4 $S --scale 50 --pose storm --only front
shot c1 $S --scale 50 --clip wind-sway --t 1 --only right
shot c2 $S --scale 50 --clip wind-sway --t 3 --only right
shot c3 $S --scale 50 --clip gust --t 2 --only right
shot c4 $S --scale 50 --clip gust --t 6 --only right
shot c5 $S --scale 50 --clip diurnal-transpiration --t 6 --only right
shot c6 $S --scale 50 --clip diurnal-transpiration --t 18 --only right
shot l1 $S --scale 50 --list thermal --cut
shot l2 $S --scale 50 --list structure
shot l3 $S --scale 200 --list thermal
shot l4 $S --scale 500 --list structure
shot k1 $S --scale 100 --list structure
shot k2 $S --scale 100 --list thermal
shot k3 $S --scale 200 --list structure
shot k4 $S --scale 500 --list thermal
shot d1 r3-2d.ts --file "$G/🔣️.json" --scale 50
shot d2 r3-2d.ts --file "$G/🔣️.json" --scale 100
shot d3 r3-2d.ts --file "$G/🔣️.json" --scale 200
shot d4 r3-2d.ts --file "$G/🔣️.json" --scale 500
