#!/bin/zsh
for i in $(seq 1 24); do
 n=$(pgrep -f "slice-lab/📜️script.ts run" | wc -l | tr -d ' ')
 [ "$n" = 0 ] && { echo "free after $i polls"; exit 0; }
 sleep 20
done
echo "still $n lab(s) running"; pgrep -fl "slice-lab/📜️script.ts run" | cut -c1-300
