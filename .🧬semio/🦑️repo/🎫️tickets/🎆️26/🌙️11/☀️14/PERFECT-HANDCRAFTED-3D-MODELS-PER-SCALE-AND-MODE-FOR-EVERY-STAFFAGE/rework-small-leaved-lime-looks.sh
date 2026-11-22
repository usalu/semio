#!/bin/sh
# 🔭 Small-leaved lime author: renders the candidate document in every pose, at two instants of every clip and in the thermal and structure lists, as three montages (poses.png, clips.png, lists.png).
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
G="🗑️generated/rework-small-leaved-lime/small-leaved-lime"
DOC="$G/🔣️.json"
shot() { name=$1; shift; out=$("$BUN" r3-sheet.ts --file "$DOC" "$@" 2>&1 | tail -1); [ -f "$out" ] && mv "$out" "$G/m-$name.png" || echo "FAILED $name $out"; }
P=""
for pose in summer-breeze drought-stress autumn-leaf-fall wet-snow gale; do
  shot "$pose-r" --scale 50 --pose $pose --only right --no-labels; shot "$pose-f" --scale 50 --pose $pose --only front --no-labels
  P="$P $G/m-$pose-r.png $G/m-$pose-f.png"
done
"$BUN" rework-small-leaved-lime-montage.ts "$G/poses.png" 5 640 $(for v in r f; do for pose in summer-breeze drought-stress autumn-leaf-fall wet-snow gale; do echo "$G/m-$pose-$v.png"; done; done)
C=""
for spec in "breeze-sway 1" "breeze-sway 3" "gust 2" "gust 5" "diurnal-transpiration 6" "diurnal-transpiration 15"; do
  set -- $spec; shot "$1-$2" --scale 50 --clip $1 --t $2 --only right --no-labels; C="$C $G/m-$1-$2.png"
done
"$BUN" rework-small-leaved-lime-montage.ts "$G/clips.png" 3 800 $C
L=""
for spec in "50 thermal" "50 structure" "100 structure" "200 thermal" "200 structure" "500 thermal" "500 structure" "100 thermal"; do
  set -- $spec; shot "$2-$1" --scale $1 --list $2 --only right --no-labels; L="$L $G/m-$2-$1.png"
done
shot "thermal-50-cut" --scale 50 --list thermal --only right --no-labels --cut
"$BUN" rework-small-leaved-lime-montage.ts "$G/lists.png" 3 800 $L "$G/m-thermal-50-cut.png"
rm -f "$G"/m-*.png
