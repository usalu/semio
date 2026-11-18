#!/bin/bash
# 🧩 Front hull set of a scale. usage: humansb-C2-front-batch.sh <scale> <outDir> <patches|->
. "$(dirname "$0")/humansb-C2-env.sh"
S=$1 OUT=$2 PT=${3:--}
cd "$N"
mk() { ./humansb-C-hullbatch.sh cyclist "$S" front "$1" "$OUT/humansb-C2-fb$S-$2.json" "$PT" replace "$2 $2 $3" >/dev/null || echo "FAIL $2"; echo "$OUT/humansb-C2-fb$S-$2.json"; }
files=()
for sd in l r; do
  files+=("$(mk thigh-$sd thigh-$sd thigh-$sd)" "$(mk shank-$sd shank-$sd shank-$sd)" "$(mk foot-$sd foot-$sd shoe-$sd)" "$(mk toe-$sd toe-$sd shoe-toe-$sd)" "$(mk crank-$sd crank-$sd crank-arm-$sd)" "$(mk pedal-$sd pedal-$sd pedal-$sd)" "$(mk frame fork-$sd fork-$sd)")
done
files+=("$(mk frame head-tube tube-head)" "$(mk frame stem stem)" "$(mk frame handlebar handlebar-l,handlebar-r,handlebar-cross,grip-l,grip-r)" "$(mk frame grip-l grip-l)" "$(mk frame grip-r grip-r)")
python3 - "$OUT/humansb-C2-fb$S.json" "${files[@]}" <<'P'
import json, sys, os
a = []
for f in sys.argv[2:]:
    if os.path.exists(f): a += json.load(open(f))
json.dump(a, open(sys.argv[1], "w")); print(len(a), "ops")
P
