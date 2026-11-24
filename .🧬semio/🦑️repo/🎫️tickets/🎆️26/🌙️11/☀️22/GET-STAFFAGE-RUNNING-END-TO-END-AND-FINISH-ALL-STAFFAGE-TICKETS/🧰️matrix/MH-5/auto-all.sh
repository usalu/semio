#!/bin/zsh
# 🚂️ Runs auto.py for the given people one after the other (fresh search each) and prints the last log line of each: auto-all.sh <id>…
cd "${0:A:h}"
for a in "$@"; do
  python3 auto.py "$a" --fresh > /dev/null 2>&1
  tail -1 "../../🗑️generated/matrix/MH-5/auto/$a.log" | cut -c1-400
done
