#!/bin/zsh
# 🚦️ Runs the MR gate (every pose, clip instant and transition, with the twin tier) for the given assets one after the other, each log in $GEN/gate/<id>.<TAG>.log: TAG=before gate-all.sh <id>…
export BATCH=MH-5
source "${0:A:h}/../MR/env.sh"
mkdir -p "$GEN/gate"
for a in "$@"; do
  s=$(date +%s)
  (cd "$T" && "$W/q.sh" timeout ${GATE_TIMEOUT:-2400} "$B" "$M/gate.ts" "${FILE:-$a}" --twins ${=GATE_ARGS}) > "$GEN/gate/$a.${TAG:-after}.log" 2>&1
  echo "$a exit $? $(( $(date +%s) - s )) s: $(tail -1 "$GEN/gate/$a.${TAG:-after}.log")"
done
