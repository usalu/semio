#!/bin/bash
# usage: vegetation-all.sh <slug>   restore the original asset -> extents/dimensions -> drawing rewrites (+ repair, manual fixes) -> triage
HERE="$(cd "$(dirname "$0")" && pwd)"
G="$HERE/vegetation-work"
slug="$1"
[ -f "$(dirname "$0")/vegetation-env-$slug.sh" ] && . "$(dirname "$0")/vegetation-env-$slug.sh"
CAT="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation"
if [ "${KEEP:-0}" != 1 ]; then cp "$G/backup/$slug.json" "$CAT/$slug/🔣️.json"; fi
[ -f "$HERE/vegetation-pre-$slug.py" ] && python3 "$HERE/vegetation-pre-$slug.py" > "$G/all-$slug.log" 2>&1
"$HERE/vegetation-bbox-fix.sh" "$slug" >> "$G/all-$slug.log" 2>&1
[ -f "$HERE/vegetation-volume-$slug.py" ] && python3 "$HERE/vegetation-volume-$slug.py" >> "$G/all-$slug.log" 2>&1
EPS=${EPS:-0.3} ROUNDS=${ROUNDS:-2} TAILN=200 "$HERE/vegetation-cycle.sh" "$slug" 0.0 ${HF:-0.25} >> "$G/all-$slug.log" 2>&1
python3 "$HERE/vegetation-repair.py" "$slug" >> "$G/all-$slug.log" 2>&1
[ -f "$HERE/vegetation-fix-$slug.py" ] && python3 "$HERE/vegetation-fix-$slug.py" >> "$G/all-$slug.log" 2>&1
[ -f "$HERE/vegetation-fix-$slug.sh" ] && "$HERE/vegetation-fix-$slug.sh" >> "$G/all-$slug.log" 2>&1
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
cd "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript"
$B "$HERE/../STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/silhouette-triage.ts" current "$G/f-$slug.jsonl" "$slug" >> "$G/all-$slug.log" 2>&1
for pass in 1 2; do
  views="$(python3 "$HERE/vegetation-failing-views.py" "$G/f-$slug.jsonl")"
  [ -z "$views" ] && break
  echo "fine pass $pass: $views" >> "$G/all-$slug.log"
  case $pass in 1) fe=${FINE_EPS:-0.12};; *) fe=${FINE_EPS2:-0.04};; esac
  "$HERE/vegetation-fine.sh" "$slug" $fe $views >> "$G/all-$slug.log" 2>&1
  python3 "$HERE/vegetation-repair.py" "$slug" >> "$G/all-$slug.log" 2>&1
  $B "$HERE/../STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/silhouette-triage.ts" current "$G/f-$slug.jsonl" "$slug" >> "$G/all-$slug.log" 2>&1
done
views="$(python3 "$HERE/vegetation-failing-views.py" "$G/f-$slug.jsonl")"
if [ -n "$views" ] && [ "${NOPATCH:-0}" != 1 ]; then
  echo "patch pass: $views" >> "$G/all-$slug.log"
  "$HERE/vegetation-patchrun.sh" "$slug" $views >> "$G/all-$slug.log" 2>&1
  [ -f "$HERE/vegetation-fix2-$slug.py" ] && python3 "$HERE/vegetation-fix2-$slug.py" >> "$G/all-$slug.log" 2>&1
  $B "$HERE/../STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/silhouette-triage.ts" current "$G/f-$slug.jsonl" "$slug" >> "$G/all-$slug.log" 2>&1
fi
python3 "$HERE/vegetation-table.py" "$G/f-$slug.jsonl" >> "$G/all-$slug.log" 2>&1
$B "$HERE/vegetation-validate.ts" "$slug" >> "$G/all-$slug.log" 2>&1
