#!/bin/sh
# 🩻 rework-tower-crane-diff.sh <scale> <view> <name> [window] : silhouette difference picture of the source merged over the catalog
cd "$(dirname "$0")"; G=🗑️generated/rework-tower-crane; mkdir -p "$G"; BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
python3 rework-tower-crane-crop.py "$1" $G/full.json ALL >/dev/null
if [ -n "$4" ]; then $BUN rework-tower-crane-diff.ts tower-crane "$1" "$2" $G/full.json $G/diff.json --ppm $G/"$3".ppm --window "$4" 2>&1 | grep DEBUG | head -3 | cut -c1-1500
else $BUN rework-tower-crane-diff.ts tower-crane "$1" "$2" $G/full.json $G/diff.json --ppm $G/"$3".ppm 2>&1 | grep DEBUG | head -3 | cut -c1-1500; fi
sips -s format png $G/"$3".ppm --out $G/"$3".png >/dev/null 2>&1 || python3 -c "print('sips failed')"; rm -f $G/"$3".ppm
