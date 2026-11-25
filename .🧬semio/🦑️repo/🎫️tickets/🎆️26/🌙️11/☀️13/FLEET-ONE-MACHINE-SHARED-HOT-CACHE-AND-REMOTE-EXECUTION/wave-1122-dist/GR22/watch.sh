#!/bin/zsh
# 👀️ Samples the fleet every 3 minutes for 30 minutes after the GR22 landing: worker stages and progress, and the selected-exchange / graph lines of the worker origins.
here=${0:A:h}; out="$here/🗑️generated/watch.txt"; cd /Users/Ueli/Documents/semio
for at in {1..11}; do
  print -r -- "=== $(date '+%H:%M:%S')" >> $out
  repo fleet status 2>&1 | grep -E 'Workers:|g1-m[0-9]+ (macos|linux|windows):|Generation' | cut -c1-260 >> $out
  for origin in /Users/Shared/semio-store/v1/origins/g1-*; do
    line=$(grep -ho 'taken [0-9]* of [0-9]*\|graph-adopted[^"]*\|module graph[^"]*\|reading the owners (round [0-9]*) [0-9]* of [0-9]*' $origin/ready.json 2>/dev/null | tail -1)
    [[ -n $line ]] && print -r -- "${origin:t} $line" >> $out
  done
  sleep 180
done
