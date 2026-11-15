#!/bin/bash
# 🪛 Runs round 27 on g1-m04: step 0 of round 16 (`repo tools setup all`, once, with exit code and wall time), then the series of tries. Usage: run-round-27.sh
set -u
M="$(cd "$(dirname "$0")" && pwd)"; log="$HOME/tools-setup-round-27.log"
export SEMIO_LANGUAGE=en SEMIO_WORKSPACE_ROOT="$HOME/semio" SEMIO_FLEET_GROUP=g1 SEMIO_FLEET_NFS_HOST=192.168.178.70 SEMIO_FLEET_NETWORK=192.168.178.0/24 PATH="$HOME/.local/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin"
cd "$HOME/semio" || exit 1
began="$(date +%s)"; echo "step 0 start $(date -u +%Y-%m-%dT%H:%M:%SZ)" > "$M/round-27.step0.txt"
repo tools setup all > "$log" 2>&1 &
pid=$!; held=0
while kill -0 "$pid" 2>/dev/null; do
  if sed $'s/\x1b\\[[0-9;]*m//g' "$log" | tail -n 3 | grep -aq 'Another process holds the dependency store'; then held=$((held + 10)); else held=0; fi
  if [ "$held" -ge 120 ]; then { echo "store held above 2 minutes: $(tail -n 1 "$log" | cut -c1-400)"; ps -eo pid,ppid,etime,args | grep -E 'bun|repo' | grep -v grep | cut -c1-260; echo "one SIGINT sent"; } >> "$M/round-27.step0.txt"; kill -INT "$pid"; break; fi
  sleep 10
done
wait "$pid"; rc=$?
{ echo "step 0 end $(date -u +%Y-%m-%dT%H:%M:%SZ) exit=$rc wall=$(( $(date +%s) - began )) s"; echo "## restored / installed lines"; sed $'s/\x1b\\[[0-9;]*m//g' "$log" | grep -aE 'restored|installed' | cut -c1-400 | tail -n 40; echo "## last 20 lines"; sed $'s/\x1b\\[[0-9;]*m//g' "$log" | tail -n 20 | cut -c1-600; } >> "$M/round-27.step0.txt"
sleep 30
bash "$M/run-tries-27.sh" 27
