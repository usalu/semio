#!/bin/zsh
# 🛡️ Stops this ticket's repo component build before the data volume fills (below 4 GiB free).
while pgrep -f "procedural-component:component" >/dev/null; do
  free=$(df -k /Users/ueli/Documents/semio | awk 'NR==2{print int($4/1048576)}')
  if [ "$free" -lt 4 ]; then echo "[DEBUG] disk-guard-repo: ${free} GiB free, stopping component build $(date)"; pkill -TERM -f "procedural-component:component"; exit 0; fi
  sleep 10
done
