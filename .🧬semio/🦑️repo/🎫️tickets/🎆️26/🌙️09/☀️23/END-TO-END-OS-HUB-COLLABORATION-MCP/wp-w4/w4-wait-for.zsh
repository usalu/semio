#!/bin/zsh
# ⏳️ W4 (14c): block until chain-final.txt gains a line matching <regex> (or the chain exits), cap <s>; then print its tail.
L=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-w4-logs
cap=${1:-570}; re=${2:-'READY|FAILED|DONE|REFUSED'}; t0=$(date +%s)
base=$(/usr/bin/grep -cE "$re" "$L/chain-final.txt")
while [ $(( $(date +%s) - t0 )) -lt $cap ]; do
  [ "$(/usr/bin/grep -cE "$re" "$L/chain-final.txt")" != "$base" ] && break
  ps -axo command= | /usr/bin/grep -qE '^zsh [^ ]*w4-chain\.sh final' || { echo "CHAIN GONE"; break; }
  sleep 15
done
echo "== waited $(( $(date +%s) - t0 ))s $(date '+%T')"; tail -12 "$L/chain-final.txt"
