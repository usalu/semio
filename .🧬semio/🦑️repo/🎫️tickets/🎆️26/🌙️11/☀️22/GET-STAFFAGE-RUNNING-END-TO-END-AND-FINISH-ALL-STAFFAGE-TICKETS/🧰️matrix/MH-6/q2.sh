#!/bin/zsh
# 🚦️ The one-at-a-time worker of the batch (the shared host allows one tool process): runs the lines of $GEN/${1:-q2}.txt in order, each to its end, waits for further lines and stops at a line STOP; every line may use $W, $M, $GEN, $B, $S, $T. Progress in $GEN/${1:-q2}.log. q.sh
export BATCH=MH-6; source "${0:A:h}/../MR/env.sh"
i=1; touch "$GEN/${1:-q2}.txt"
while true; do
  line="$(sed -n "${i}p" "$GEN/${1:-q2}.txt")"
  if [[ -z "$line" ]]; then sleep 5; continue; fi
  [[ "$line" == STOP ]] && break
  s=$SECONDS; echo "start $i: $line" >> "$GEN/${1:-q2}.log"; eval "$line"; echo "done $i exit $? $((SECONDS-s))s" >> "$GEN/${1:-q2}.log"; i=$((i+1))
done
