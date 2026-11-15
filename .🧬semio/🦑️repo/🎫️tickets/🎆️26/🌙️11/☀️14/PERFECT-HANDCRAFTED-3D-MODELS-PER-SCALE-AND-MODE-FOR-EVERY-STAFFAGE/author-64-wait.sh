#!/bin/bash
# ⏳ Author 64: rewrites decisions of assets once nobody touched their source for 20 minutes, then splices. Gives up after 55 minutes.
cd "$(dirname "$0")"
TARGETS="animals/horse animals/dairy-cow animals/labrador-dog humans/walking-commuter humans/standing-phone-user humans/sleeping-person humans/person-cooking humans/person-with-rollator vegetation/english-oak vegetation/hosta-clump vegetation/agave"
DONE=""
for i in $(seq 1 55); do
  for t in $TARGETS; do
    case " $DONE " in *" $t "*) continue;; esac
    out=$(python3 author-64-decisions.py "$t" 2>&1)
    case "$out" in WROTE*) ./author-64-splice.sh "$t" | grep SPLICED; DONE="$DONE $t";; esac
  done
  python3 author-64-walls.py >/dev/null 2>&1 && python3 - <<'PY' >/dev/null 2>&1
PY
  [ $(echo $DONE | wc -w) -eq $(echo $TARGETS | wc -w) ] && break
  sleep 60
done
echo "DONE:$DONE"
