#!/bin/sh
cd /Users/ueli/Documents/semio
TF=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
L="$TF/🗑️generated/wgpu-frame-job-test.txt"
sh "$TF/wgpu-frame-job-test.sh" &
low=0
while ! grep -aq "frame-job test exit" "$L" 2>/dev/null; do
  f=$(memory_pressure | tail -1 | grep -o '[0-9]*%' | tr -d %)
  s=$(sysctl -n vm.swapusage | awk '{gsub("M","",$6); print int($6)}')
  d=$(df -g /System/Volumes/Data | tail -1 | awk '{print $4}')
  if [ "$f" -lt 30 ]; then low=$((low+1)); else low=0; fi
  if [ $low -ge 6 ] || [ "$s" -gt 4000 ] || [ "$d" -lt 9 ]; then
    for p in $(ps -axww -o pid=,args= | grep -a "cargo test" | grep -a "frame-job" | grep -v grep | awk '{print $1}'); do kill -INT $p; done
    echo "GUARD free=$f swap=$s disk=$d" >> "$L"; break
  fi
  sleep 10
done
wait
