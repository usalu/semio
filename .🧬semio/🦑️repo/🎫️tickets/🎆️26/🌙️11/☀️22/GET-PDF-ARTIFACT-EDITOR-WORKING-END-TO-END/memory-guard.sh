#!/bin/sh
set -u
floor=${MEMORY_GUARD_FREE_PERCENT:-25}
watch=${MEMORY_GUARD_WATCH_PERCENT:-45}
ceiling=${MEMORY_GUARD_PROCESS_MB:-8200}
log="$(dirname -- "$0")/🗑️generated/memory-guard.log"
mkdir -p "$(dirname -- "$log")"
megabytes() {
  awk '{ v = $3; n = v + 0; if (v ~ /G/) n = n * 1024; else if (v ~ /K/) n = n / 1024; printf "%d %s %s\n", n, $1, $2 }'
}
while :; do
  free=$(memory_pressure 2>/dev/null | awk -F': ' '/free percentage/ { gsub("%", "", $2); print $2 }')
  biggest=$(top -l 1 -o mem -n 1 -stats pid,command,mem 2>/dev/null | tail -1 | megabytes)
  size=$(echo "$biggest" | awk '{print $1}')
  pid=$(echo "$biggest" | awk '{print $2}')
  name=$(echo "$biggest" | awk '{print $3}')
  if [ -n "$free" ] && [ "$free" -lt "$watch" ]; then
    echo "$(date '+%F %T') free=$free% biggest: $pid $name ${size}M" >> "$log"
  fi
  if [ -n "$size" ] && [ "$size" -gt "$ceiling" ] && [ "$name" = "rustc" ]; then
    echo "$(date '+%F %T') free=$free% killing rustc $pid at ${size}M (ceiling ${ceiling}M): $(ps -o command= -p "$pid" | grep -oE -- '--crate-name [a-z0-9_]+')" >> "$log"
    kill -9 "$pid" 2>/dev/null
    sleep 10
  elif [ -n "$free" ] && [ "$free" -lt "$floor" ]; then
    victim=$(ps -Ao pid=,rss=,command= | grep -E 'native-target/lane-|/rustc ' | grep -v grep | sort -k2 -n -r | head -1)
    vpid=$(echo "$victim" | awk '{print $1}')
    if [ -n "$vpid" ]; then
      echo "$(date '+%F %T') free=$free% killing $(echo "$victim" | cut -c1-160)" >> "$log"
      kill -9 "$vpid" 2>/dev/null
      sleep 20
    fi
  fi
  sleep 3
done
