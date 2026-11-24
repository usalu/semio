#!/bin/zsh
# 🚦️ Runs the MR gate (every pose, clip instant and transition, with the twin tier) for the given asset ids or documents one after the other; the log of each goes to $GEN/gate/<name>.<TAG>.log: TAG=before gate-all.sh <id|file.json>…
export BATCH=MH-3
source "${0:A:h}/../MR/env.sh"
mkdir -p "$GEN/gate"
for a in "$@"; do
  n="${${a:t}%.json}"
  (cd "$T" && timeout 1500 "$B" "$M/gate.ts" "$a" --twins ${=GATE_FLAGS}) > "$GEN/gate/$n.${TAG:-now}.log" 2>&1
  echo "$n exit $? $(tail -1 "$GEN/gate/$n.${TAG:-now}.log")"
done
