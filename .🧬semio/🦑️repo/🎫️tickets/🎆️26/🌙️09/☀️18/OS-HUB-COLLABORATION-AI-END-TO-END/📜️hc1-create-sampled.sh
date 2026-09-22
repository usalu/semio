#!/bin/zsh
# 🩺️ HC1 — run C8's artifact-creation diagnose against a live hub while sampling the hub
# process's CPU every 2 s, so "the hub burns no CPU" is measured on the CHILD that listens on the
# port rather than on the bun holder that supervises it.
# Usage: 📜️hc1-create-sampled.sh <origin> <port> <email> <password> <capture>
set -u
origin="$1"; port="$2"; email="$3"; password="$4"; capture="$5"
root="${0:A:h}/../../../../../../.."
cd "${0:A:h}"
hub_pid=$(lsof -nP -iTCP:"$port" -sTCP:LISTEN 2>/dev/null | awk 'NR==2{print $2}')
{
  echo "HUB PID $hub_pid  started $(date '+%H:%M:%S')  load $(uptime | sed 's/.*load averages: //')"
} > "$capture"
(
  while true; do
    line=$(ps -p "$hub_pid" -o pcpu=,rss= 2>/dev/null)
    [[ -z "$line" ]] && break
    echo "CPU $(date '+%H:%M:%S') $line"
    sleep 2
  done
) >> "$capture" 2>&1 &
sampler=$!
start=$(date +%s)
bun "🐍️c8-create-diagnose.ts" "$origin" "$email" "$password" >> "$capture" 2>&1
end=$(date +%s)
kill "$sampler" 2>/dev/null
echo "WALL $((end - start))s  ended $(date '+%H:%M:%S')" >> "$capture"
