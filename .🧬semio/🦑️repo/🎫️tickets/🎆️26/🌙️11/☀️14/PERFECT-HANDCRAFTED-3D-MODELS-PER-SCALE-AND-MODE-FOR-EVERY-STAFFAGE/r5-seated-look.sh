#!/bin/sh
# 🔭 Round 5, seated figures: rebuilds the candidate of one asset (office|reader) and renders the review montages under 🗑️generated/r5-seated/<who>/: body of the four scales, structure of the four scales, thermal cut of the four scales, the poses at 1:50 and one clip frame.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
who=$1
case "$who" in office) slug=seated-office-worker ;; reader) slug=seated-reader ;; *) echo "usage: r5-seated-look.sh office|reader [tag]"; exit 2 ;; esac
tag=${2:-v}
F="🗑️generated/r3-$slug/🔣️.json"
O="🗑️generated/r5-seated/$who"
mkdir -p "$O"
"$BUN" "r3-$slug-build.ts" > /dev/null
sheet() { "$BUN" "r3-$slug-sheet.ts" "humans/$slug" --file "$F" "$@" 2>&1 | tail -1; }
for list in body structure thermal; do
  files=""
  for s in 50 100 200 500; do
    extra=""; [ "$list" = thermal ] && extra="--cut"; [ "$list" = body ] && extra="--no-rig"
    files="$files $(sheet --scale $s --list $list $extra)"
  done
  "$BUN" r5-seated-montage.ts "$O/$tag-$list.png" $files > /dev/null
  echo "$O/$tag-$list.png"
done
poses=$(python3 -c "import json;a=json.load(open('$F'))['asset'];print(' '.join(p['id'] for p in a['poses']))")
clip=$(python3 -c "import json;a=json.load(open('$F'))['asset'];c=a['clips'][-1];print(c['id'], round(c.get('durationS', 2)*0.6, 2))")
files=""
for p in $poses; do files="$files $(sheet --scale 50 --no-rig --pose $p)"; done
set -- $clip
files="$files $(sheet --scale 50 --no-rig --clip $1 --t $2)"
"$BUN" r5-seated-montage.ts "$O/$tag-poses.png" $files > /dev/null
echo "$O/$tag-poses.png ($poses, clip $1 at $2 s)"
