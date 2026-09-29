#!/bin/zsh
# ⏳️ LW1: block ≤ 570 s until a capture has its LW1-END line; prints the tail.
out="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lw1-logs/$1.txt"; n=0
while ! /usr/bin/grep -q '^LW1-END' "$out" 2>/dev/null && [ $n -lt 57 ]; do sleep 10; n=$((n + 1)); done
/usr/bin/grep -E '^LW1-' "$out"; echo "--- waited $((n * 10))s load=$(sysctl -n vm.loadavg)"
