#!/bin/sh
# 🎞️ Finisher review: renders, for one candidate, the body sheet of every scale, the thermal and structure sheets at 1:50, every pose at 1:50, two instants of every clip and the 2D drawings of every scale, one process at a time, and lays them out as contact sheets under 🗑️generated/review/<slug>.
cd "$(dirname "$0")" || exit 1
K="$PWD"; slug=$1; C="$K/assets/$slug/🔣️.json"; OUT="$K/🗑️generated/review/$slug"; mkdir -p "$OUT"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
T="$K/../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
cd "$T" || exit 1
G="$T/🗑️generated"
rm -f "$G/r3-sheet/file/$slug".*.png "$G/r3-2d/file/$slug".*.png
for scale in 50 100 200 500; do
  "$BUN" r3-sheet.ts --file "$C" --scale $scale > /dev/null 2>> "$OUT/errors.txt"
  "$BUN" r3-sheet.ts --file "$C" --scale $scale --list thermal > /dev/null 2>> "$OUT/errors.txt"
  "$BUN" r3-sheet.ts --file "$C" --scale $scale --list structure > /dev/null 2>> "$OUT/errors.txt"
  "$BUN" r3-2d.ts --file "$C" --scale $scale > /dev/null 2>> "$OUT/errors.txt"
done
python3 - "$C" <<'PY' > "$OUT/states.txt"
import json, sys
asset = json.load(open(sys.argv[1]))["asset"]
for pose in asset["poses"]:
    if pose["id"] != asset["presentation"]["defaultPose"]:
        print("pose", pose["id"])
for clip in asset["clips"]:
    print("clip", clip["id"], round(clip["durationS"] * 0.35, 2))
    print("clip", clip["id"], round(clip["durationS"] * 0.8, 2))
PY
while read kind id t; do
  if [ "$kind" = pose ]; then "$BUN" r3-sheet.ts --file "$C" --scale 50 --pose "$id" > /dev/null 2>> "$OUT/errors.txt"
  else "$BUN" r3-sheet.ts --file "$C" --scale 50 --clip "$id" --t "$t" > /dev/null 2>> "$OUT/errors.txt"; fi
done < "$OUT/states.txt"
S="$G/r3-sheet/file/$slug"; D="$G/r3-2d/file/$slug"
cd "$K" || exit 1
"$BUN" contact.ts "$OUT/body.png" "$S.s50.body.png" "$S.s100.body.png" "$S.s200.body.png" "$S.s500.body.png"
"$BUN" contact.ts "$OUT/thermal.png" "$S.s50.thermal.png" "$S.s100.thermal.png" "$S.s200.thermal.png" "$S.s500.thermal.png"
"$BUN" contact.ts "$OUT/structure.png" "$S.s50.structure.png" "$S.s100.structure.png" "$S.s200.structure.png" "$S.s500.structure.png"
"$BUN" contact.ts "$OUT/2d.png" "$D.s50.png" "$D.s100.png" "$D.s200.png" "$D.s500.png"
ls "$S".s50.body.*.png | split -l 4 - "$OUT/states-"
n=0; for part in "$OUT"/states-*; do n=$((n+1)); "$BUN" contact.ts "$OUT/states$n.png" $(cat "$part" | tr '\n' ' '); rm "$part"; done
[ -s "$OUT/errors.txt" ] && cat "$OUT/errors.txt"
ls "$OUT"
