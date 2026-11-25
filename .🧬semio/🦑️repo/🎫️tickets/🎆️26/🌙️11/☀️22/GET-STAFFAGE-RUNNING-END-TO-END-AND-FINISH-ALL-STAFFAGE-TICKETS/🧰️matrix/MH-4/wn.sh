#!/bin/zsh
# ⏲️ Waits at most 170 s until a file holds more lines that match a pattern than the count given, then prints the time and the count: wn.sh <file> <pattern> <count>
n=0
until [[ $n -ge 34 || "$(grep -c "$2" "$1" 2>/dev/null)" -gt "$3" ]]; do sleep 5; n=$((n+1)); done
echo "$(date +%H:%M) $(grep -c "$2" "$1" 2>/dev/null)"
