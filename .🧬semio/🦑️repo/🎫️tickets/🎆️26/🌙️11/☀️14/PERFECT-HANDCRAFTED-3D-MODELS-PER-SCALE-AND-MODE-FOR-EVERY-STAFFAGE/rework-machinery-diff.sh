#!/bin/sh
# 🩻 Silhouette difference picture of any asset (source merged over the catalog) with `rework-tower-crane-diff.ts`: `sh rework-machinery-diff.sh <category>/<slug> <scale> <view> <name> [window x0,y0,x1,y1]`; grey = agree, red = drawn only, blue = body only; the picture lands in `🗑️generated/rework-machinery/<name>.png`.
cd "$(dirname "$0")" || exit 2
G=🗑️generated/rework-machinery; mkdir -p "$G"; BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1); SLUG=${1#*/}
python3 - "$1" <<'P'
import json,glob,sys
p=sys.argv[1]; slug=p.split('/')[1]
f=glob.glob('/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/'+slug+'/🔣️.json')[0]
d=json.load(open(f)); d['asset']['volumes']=json.load(open('volumes/'+p+'.json'))['volumes']
json.dump(d,open('🗑️generated/rework-machinery/full.json','w'),ensure_ascii=False)
P
if [ -n "$5" ]; then W="--window $5"; fi
$BUN rework-tower-crane-diff.ts "$SLUG" "$2" "$3" $G/full.json $G/diff.json --ppm $G/"$4".ppm $W 2>&1 | grep -v "^$" | head -4 | cut -c1-600
sips -s format png $G/"$4".ppm --out $G/"$4".png >/dev/null 2>&1; rm -f $G/"$4".ppm; ls $G/"$4".png
