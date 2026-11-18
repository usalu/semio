#!/bin/bash
# 💪 Front arm hulls of a scale. usage: humansb-C2-arms-batch.sh <scale> <out.json> <patches|-> <forearm-solid-prefix: forearm|forearm-hand>
. "$(dirname "$0")/humansb-C2-env.sh"
S=$1 OUT=$2 PT=$3 FS=$4
cd "$N"
fs=()
for sd in l r; do
  for spec in "arm-$sd upper-arm-$sd sleeve-$sd" "forearm-$sd forearm-$sd $FS-$sd" "hand-$sd hand-$sd hand-$sd"; do
    read -r bone id solids <<<"$spec"
    f="$G/humansb-C2-ab$S-$id.json"
    ./humansb-C-hullbatch.sh cyclist "$S" front "$bone" "$f" "$PT" replace "$id $id $solids" >/dev/null || echo "FAIL $id"
    fs+=("$f")
  done
done
python3 - "$OUT" "${fs[@]}" <<'P'
import json, sys, os
a = []
for f in sys.argv[2:]:
    if os.path.exists(f): a += json.load(open(f))
json.dump(a, open(sys.argv[1], "w")); print(len(a), "ops")
P
