#!/bin/bash
P=$1; OUT=$2
CG=/sys/fs/cgroup$(sed 's/0:://' /proc/$P/cgroup)
while kill -0 $P 2>/dev/null; do
  a=$(grep -E "^anon " $CG/memory.stat | cut -d' ' -f2); f=$(grep -E "^file " $CG/memory.stat | cut -d' ' -f2); s=$(grep -E "^slab " $CG/memory.stat | cut -d' ' -f2)
  echo "$(date +%T) anonMB=$((a/1048576)) fileMB=$((f/1048576)) slabMB=$((s/1048576)) swap=$(grep VmSwap /proc/$P/status | awk '{print $2/1024}')" >> "$OUT"
  sleep 15
done
echo "$(date +%T) exited" >> "$OUT"
