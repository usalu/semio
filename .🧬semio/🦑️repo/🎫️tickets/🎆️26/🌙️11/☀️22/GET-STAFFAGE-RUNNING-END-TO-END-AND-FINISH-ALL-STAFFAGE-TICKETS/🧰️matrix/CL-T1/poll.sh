#!/bin/zsh
# 🛎️ Waits up to 170 seconds for a worker of lane CL-T1 to finish another job and prints one line of state: the time, the proved assets, the failed light proofs, the last finished job of each worker and the assets whose model-catalog run still names findings. poll.sh
source "${0:A:h}/env.sh"
c() { cat "$GEN/log/jobs-done.txt" "$GEN/log/jobs2-done.txt" "$GEN/log/a-done.txt" "$GEN/log/b-done.txt" 2>/dev/null | wc -l | tr -d ' ' }
b=$(c); n=0; until [[ $(c) != $b || $n -ge 34 ]]; do sleep 5; n=$((n+1)); done
echo "$(date +%H:%M) proved $(ls "$GEN"/proof/*.proved 2>/dev/null | wc -l | tr -d ' '); light FAIL: $(grep -l 'LIGHT PROOF.*FAIL' "$GEN"/light/*.txt 2>/dev/null | xargs -n1 basename 2>/dev/null | tr '\n' ' '); a: $(cat "$GEN/log/jobs-done.txt" "$GEN/log/a-done.txt" 2>/dev/null | tail -1 | cut -c1-80); b: $(cat "$GEN/log/jobs2-done.txt" "$GEN/log/b-done.txt" 2>/dev/null | tail -1 | cut -c1-80)"
for f in "$GEN"/proof/*-modelcat.txt(N); do [[ -s "$f" ]] && echo "  ${${f:t}%-modelcat.txt}: $(wc -l < "$f" | tr -d ' ') findings, first: $(head -1 "$f" | cut -c1-110)"; done
