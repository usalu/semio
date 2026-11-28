#!/bin/zsh
# ⏳️ Waits until a worker of lane CL-T1 finishes another job (or 170 seconds pass), then prints the state (`status.sh`). wait.sh
source "${0:A:h}/env.sh"
c() { cat "$GEN/log/jobs-done.txt" "$GEN/log/jobs2-done.txt" "$GEN/log/a-done.txt" "$GEN/log/b-done.txt" 2>/dev/null | wc -l | tr -d ' ' }
b=$(c); n=0; until [[ $(c) != $b || $n -ge 34 ]]; do sleep 5; n=$((n+1)); done
zsh "$W/status.sh" | tail -n ${1:-40}
