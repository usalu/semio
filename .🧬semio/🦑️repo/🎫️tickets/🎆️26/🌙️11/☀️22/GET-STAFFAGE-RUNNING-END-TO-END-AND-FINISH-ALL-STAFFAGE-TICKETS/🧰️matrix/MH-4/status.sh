#!/bin/zsh
# 📟️ Where the queue of the batch stands: status.sh [wait-seconds]   waits (at most the seconds given) until an output file of do.sh or finish.sh changes, then prints per asset the last lines of its newest dry run or finish, without the per-scale lines.
source "${0:A:h}/env.sh"
stamp() { stat -f %m "$GEN"/do/*.out "$GEN"/proof/*.out 2>/dev/null | sort -n | tail -1 }
s0="$(stamp)"; t=0
while [[ "$(stamp)" == "$s0" && $t -lt ${1:-0} ]]; do sleep 5; t=$((t+5)); done
date +%H:%M
for f in "$GEN"/do/*.out; do
  echo "## ${f:t:r} $(stat -f %Sm -t %H:%M "$f")"
  grep -v "subjects\|bbox\|^    \|^sheet \|rest-probe\|^[0-9]*$" "$f" | cut -c1-${COLS:-260} | tail -${LINES_EACH:-6}
done
ls "$GEN/devq" 2>/dev/null | tr '\n' ' '; ls -d "$GEN/lock" 2>/dev/null | sed 's/.*/locked/'
