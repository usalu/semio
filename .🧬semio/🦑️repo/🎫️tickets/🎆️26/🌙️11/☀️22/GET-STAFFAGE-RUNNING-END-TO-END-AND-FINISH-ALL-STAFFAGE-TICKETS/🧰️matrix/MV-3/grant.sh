#!/bin/zsh
# 🎟️ Runs one script of batch MV-3 with all its tool processes, one after the other, under ONE grant of the machine-wide memory queue, so a chain of small steps waits in the crowded queue once instead of once per step: grant.sh <megabytes of the largest step> <timeout seconds of the whole chain> <script of this folder> <args…>. Inside the grant the scripts run their steps directly (`MV3_GRANT=1`, see the function `q` in them); without it every step asks the queue itself through `q.sh`.
export BATCH=MV-3; source "${0:A:h}/../MR/env.sh"
mb="$1"; t="$2"; script="$3"; shift 3
"$W/q.sh" "$mb" "$W" "$t" env MV3_GRANT=1 zsh "$W/$script" "$@"
