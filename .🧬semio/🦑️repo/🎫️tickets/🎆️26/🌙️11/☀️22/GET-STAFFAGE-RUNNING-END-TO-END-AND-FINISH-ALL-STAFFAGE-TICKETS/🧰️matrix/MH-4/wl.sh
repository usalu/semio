#!/bin/zsh
# ⏲️ Waits at most 170 s until $GEN/final.log has more lines than the count given, then prints its last lines: wl.sh <count>
source "${0:A:h}/env.sh"
touch "$GEN/final.log"; n=0
until [[ $n -ge 34 || "$(grep -c "" "$GEN/final.log")" -gt "$1" ]]; do sleep 5; n=$((n+1)); done
echo "$(date +%H:%M) $(grep -c "" "$GEN/final.log") | $(tail -3 "$GEN/final.log" | tr '\n' '|')"
