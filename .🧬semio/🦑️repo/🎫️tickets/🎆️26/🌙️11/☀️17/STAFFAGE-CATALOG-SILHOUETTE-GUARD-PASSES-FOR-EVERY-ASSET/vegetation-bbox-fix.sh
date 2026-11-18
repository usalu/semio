#!/bin/bash
# usage: vegetation-bbox-fix.sh <slug>   dims + extent shifts (two passes)
HERE="$(cd "$(dirname "$0")" && pwd)"
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
G="$HERE/vegetation-work"
slug="$1"
cd "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript"
for pass in 1 2 3 4; do
  JSON=1 $B "$HERE/vegetation-bbox.ts" "$slug" > "$G/bbox-$slug.jsonl"
  python3 "$HERE/vegetation-bbox-plan.py" "$G/bbox-$slug.jsonl" > "$G/bboxplan-$slug.txt"
  if true; then python3 "$HERE/vegetation-dims-apply.py" "$slug" "$G/bboxplan-$slug.txt"; JSON=1 $B "$HERE/vegetation-bbox.ts" "$slug" > "$G/bbox-$slug.jsonl"; python3 "$HERE/vegetation-bbox-plan.py" "$G/bbox-$slug.jsonl" > "$G/bboxplan-$slug.txt"; fi
  : > "$G/shifts-$slug.jsonl"
  grep "^SHIFT" "$G/bboxplan-$slug.txt" | while read -r _ _ scale axis side shift _; do
    $B "$HERE/vegetation-extent.ts" "$slug" "$scale" "$axis" "$side" "$shift" >> "$G/shifts-$slug.jsonl"
  done
  [ -s "$G/shifts-$slug.jsonl" ] && python3 "$HERE/vegetation-extent-apply.py" "$slug" "$G/shifts-$slug.jsonl"
done
JSON=1 $B "$HERE/vegetation-bbox.ts" "$slug" > "$G/bbox-$slug.jsonl"
python3 "$HERE/vegetation-dims.py" "$G/bbox-$slug.jsonl"
