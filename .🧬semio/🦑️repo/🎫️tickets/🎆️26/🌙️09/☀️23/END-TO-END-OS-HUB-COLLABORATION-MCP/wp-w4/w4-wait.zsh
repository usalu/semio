#!/bin/zsh
# ⏳️ W4 (14c): block until the chain log or the wasm-hold log gains a line (or the chain dies), cap $1 s (default 570), then print both tails.
L=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-w4-logs
cap=${1:-570}; t0=$(date +%s)
n() { cat "$L/chain-final.txt" "$L/final-wasm-hold.txt" 2>/dev/null | wc -l; }
base=$(n)
while [ $(( $(date +%s) - t0 )) -lt $cap ]; do
  [ "$(n)" != "$base" ] && break
  cp=$(ps -axo pid=,command= | /usr/bin/grep -E "zsh [^ ]*w4-chain\.sh final" | /usr/bin/grep -v grep | awk '{print $1}' | head -1)
  [ -n "$cp" ] && seen=1
  [ -n "${seen:-}" ] && [ -z "$cp" ] && { echo "CHAIN GONE"; break; }
  sleep 15
done
echo "== waited $(( $(date +%s) - t0 ))s $(date '+%T') load=$(sysctl -n vm.loadavg) rustc=$(ps -axo command | /usr/bin/grep -c '^[^ ]*rustc ')"
echo "== chain"; tail -6 "$L/chain-final.txt"
echo "== hold"; tail -8 "$L/final-wasm-hold.txt"
echo "== rebuild-all"; /usr/bin/grep -oE 'rebuild-all [0-9]+/[0-9]+ [a-z0-9-]+ \([a-z]+\) (started|done in [0-9]+ s)' "$L/final-rebuild-all.txt" 2>/dev/null | tail -3
/usr/bin/grep -m5 -E 'could not compile|^error' "$L/final-rebuild-all.txt" 2>/dev/null
exit 0
