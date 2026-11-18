#!/bin/bash
# usage: vegetation-cycle.sh <slug> <iou> <hf> [apply-args...]   (trace -> apply -> refine) x ROUNDS -> triage -> table
HERE="$(cd "$(dirname "$0")" && pwd)"
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
OLD="$HERE/../STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS"
G="$HERE/vegetation-work"
slug="$1"; shift
cd "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript"
for round in $(seq 1 ${ROUNDS:-2}); do
  $B "$HERE/vegetation-trace.ts" "$slug" "$G/trace-$slug.json" | tail -1
  python3 "$HERE/vegetation-apply.py" "$slug" "$G/trace-$slug.json" "$@" | tail -${TAILN:-3}
  for i in 1 2 3 4; do
    $B "$HERE/vegetation-refine.ts" "$slug" "$G/edits-$slug.json" "$G/corr-$slug.json" | tail -1
    if [ "$(python3 -c "import json;print(len(json.load(open('$G/corr-$slug.json'))))")" = "0" ]; then break; fi
    python3 "$HERE/vegetation-apply.py" "$slug" --corrections "$G/corr-$slug.json" | tail -1
  done
done
$B "$OLD/silhouette-triage.ts" current "$G/c-$slug.jsonl" "$slug"
python3 "$HERE/vegetation-table.py" "$G/c-$slug.jsonl"
