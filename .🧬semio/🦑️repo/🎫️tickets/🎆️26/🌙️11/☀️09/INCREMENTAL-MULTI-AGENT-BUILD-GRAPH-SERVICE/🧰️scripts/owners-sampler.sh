#!/usr/bin/env bash
# samples the largest bun process (job) and the available host memory every 3 s: epoch, rss MB, available MB
while true; do
 r=$(ps -eo rss,cmd --sort=-rss | grep "[b]un" | head -1 | awk '{printf "%d",$1/1024}')
 a=$(free -m | awk 'NR==2{print $7}')
 echo "$(date +%s) $r $a"; sleep 3
done
