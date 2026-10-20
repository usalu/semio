#!/bin/sh
# 🧪 Re-plans one hub, rebuilds the overlays, runs the oracle on pre/post and the differential. One process at a time, ~12 minutes for the umbrella.
# usage: sh overlay.sh <hub> [tag]   (read-only on the checkout; writes /private/tmp/gran-hubs and the generated folder)
set -eu
H="$(cd "$(dirname "$0")" && pwd)"
A="$H/../analysis"
hub="$1"; tag="${2:-$1}"
O=/private/tmp/gran-hubs
G="$H/../../🗑️generated/gran-hubs/oracle"
nice -n 10 python3 "$A/plan_hub.py" "$hub" | tail -2
nice -n 10 python3 "$A/assemble_plan.py" "$hub" --tag "$tag" | tail -2
nice -n 10 python3 "$A/mkoverlay.py" "$tag" "$hub" 14 | tail -6
nice -n 10 python3 "$H/oracle.py" run "$O/overlay/pre" "$O/fam/$tag.json" pre --cfgs lib > "$O/oracle-pre.log" 2>&1
nice -n 10 python3 "$H/oracle.py" run "$O/overlay/post" "$O/fam/$tag.json" post --cfgs lib > "$O/oracle-post.log" 2>&1
echo "== pre vs post"; python3 "$H/oracle_diff.py" "$G/gh-${hub}fam-pre-cur-lib-final.err" "$G/gh-${hub}fam-post-cur-lib-final.err" | head -6
