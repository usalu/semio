#!/bin/zsh
# 🤖️ The greedy silhouette repair over people and scales that have no finished log yet (a log is finished when it ends in its `final` line), the finest scale first, until about TOTAL seconds are used; continues the ops of 🗑️generated/REP-B/ops (OPSDIR and LOGDIR name other folders; PAIRS="slug-scale …" limits the run to those). resume.sh <slug>…
source "${0:A:h}/env.sh"
OPSDIR="${OPSDIR:-$GEN/ops}"; LOGDIR="${LOGDIR:-$GEN/auto}"
mkdir -p "$OPSDIR" "$LOGDIR"
cd "$S/📦️packages/🟦️typescript"
start=$SECONDS
for a in "$@"; do
  for s in ${=SCALES:-50 100 200 500}; do
    log="$LOGDIR/$a-$s.log"
    [[ -n "$PAIRS" && " $PAIRS " != *" $a-$s "* ]] && continue
    [[ -f "$log" ]] && tail -1 "$log" | grep -q "final " && continue
    (( SECONDS - start > ${TOTAL:-420} )) && { echo "RESUME-MORE"; exit 0; }
    MAXSEC=${MAXSEC:-150} "$B" "$W/auto.ts" "$a" $s "$OPSDIR/$a.json" ${MAXITER:-60} "$OPSDIR/$a.json" > "$log" 2>&1
    echo "$a 1:$s $(grep 'issues ' "$log" | tail -1 | cut -c9-); $(tail -1 "$log" | cut -c9-200)"
  done
done
echo RESUME-DONE
