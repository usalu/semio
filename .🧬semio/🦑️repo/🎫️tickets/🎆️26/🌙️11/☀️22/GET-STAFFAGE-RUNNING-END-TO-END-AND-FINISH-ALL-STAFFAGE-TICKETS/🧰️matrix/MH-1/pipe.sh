#!/bin/zsh
# 🏭️ One asset from the measurement to the trial: the ops of the hard bones (pick.py from the baseline gate and the proposals, plus extra/<id>.json), the silhouette tuner (tune.py), the dry run on a copy (dry.sh) and the gate and measures of the copy (trial.sh): pipe.sh <id>…   (NOPICK=1 keeps ops/<id>.json as it is and only tunes and tries)
export BATCH=MH-1; source "${0:A:h}/../MR/env.sh"; cd "$W"
for a in "$@"; do
  echo "=== $a"
  if [[ -z "$NOPICK" ]]; then python3 pick.py "$a" "$GEN/gate/before-$a.log" "$GEN/suggest/$a.json" 2>/dev/null | python3 mk.py "$a" $([[ -f extra/$a.json ]] && echo extra/$a.json); fi
  rm -f "$GEN/tune/$a.log"; python3 tune.py "$a" | grep -v " no$\|refused"
  ./dry.sh "$a" | grep "REFUSED\|ops,"
  ./trial.sh "$a" 2>&1 | cut -c1-260
done
