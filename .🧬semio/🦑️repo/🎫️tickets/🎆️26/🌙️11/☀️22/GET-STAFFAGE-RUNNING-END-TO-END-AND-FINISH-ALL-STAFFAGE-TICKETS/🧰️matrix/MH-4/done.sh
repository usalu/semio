#!/bin/zsh
# ⏳️ Waits (at most the seconds given, default 170) until one more finish has ended, then prints per finished asset its matrix line and time: done.sh [seconds]
source "${0:A:h}/env.sh"
count() { grep -l "proof took" "$GEN"/proof/*.out 2>/dev/null | wc -l | tr -d ' ' }
stamp() { stat -f %m "$GEN"/proof/*.out 2>/dev/null | sort -n | tail -1 }
c0="$(count)$(grep -c "proof took" "$GEN"/proof/*.out 2>/dev/null | tr '\n' ' ')"; t=0
while [[ "$(count)$(grep -c "proof took" "$GEN"/proof/*.out 2>/dev/null | tr '\n' ' ')" == "$c0" && $t -lt ${1:-170} ]]; do sleep 5; t=$((t+5)); done
date +%H:%M
for f in "$GEN"/proof/*.out; do echo "${f:t:r} $(stat -f %Sm -t %H:%M "$f") | $(grep -E "^matrix exit|proof took" "$f" | tail -2 | tr '\n' ' ' | cut -c1-170)"; done
grep -h "^STOP" "$GEN"/do/*.finish.out 2>/dev/null | tail -5
