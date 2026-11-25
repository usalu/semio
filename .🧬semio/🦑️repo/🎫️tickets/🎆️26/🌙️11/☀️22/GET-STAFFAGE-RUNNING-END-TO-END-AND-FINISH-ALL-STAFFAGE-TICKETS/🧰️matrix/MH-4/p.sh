#!/bin/zsh
# ⏱️ Waits (at most 170 s) until the chain of all.sh ends one more proof or stops an asset, then prints one line per ended proof and the last line of the chain: p.sh
source "${0:A:h}/env.sh"
k() { grep -c "proof took\|^STOP\|finished" "$GEN/all.out" 2>/dev/null }
k0=$(k); t=0
while [[ "$(k)" == "$k0" && $t -lt 170 ]]; do sleep 5; t=$((t+5)); done
echo "$(date +%H:%M) $(grep "proof took\|^STOP\|^matrix exit\|finished" "$GEN/all.out" | cut -c1-130 | tail -${1:-3} | tr '\n' '|') :: $(tail -1 "$GEN/all.out" | cut -c1-90)"
