#!/bin/zsh
# 📈️ Samples every 10 s the resident memory of every process whose command line names the given text (the matrix output folder of an asset) and appends "<time> <processes> <sum MB> <largest MB>" to the given log until no such process is left for 60 s after the first was seen: rss.sh <text> <log>
seen=0; idle=0
while true; do
  line="$(ps -axo rss=,command= | grep -F -- "$1" | grep -v -e "grep" -e "rss.sh" | awk '{s+=$1; if($1>m)m=$1; n++} END{if(n)printf "%d %d %d", n, s/1024, m/1024}')"
  if [[ -n "$line" ]]; then seen=1; idle=0; echo "$(date +%H:%M:%S) $line" >> "$2"; else idle=$((idle+10)); fi
  [[ $seen == 1 && $idle -ge 60 ]] && break
  sleep 10
done
