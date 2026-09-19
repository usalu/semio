#!/bin/sh
# 🔁️ H1: retries 📜️h1-dev-boot.sh until a green workspace window appears. The fleet rewrites shared
# framework crates continuously, so a single `os-hub:dev` attempt routinely dies on a peer's half-saved
# file; each attempt is recorded and the loop stops at the first run that reaches /readyz twice.
# Usage: 📜️h1-dev-boot-retry.sh <capture-dir> <port> <attempts> [per-attempt-wait-seconds]
set -eu
capture=$1
port=$2
attempts=$3
waits=${4:-2400}
here=$(cd "$(dirname "$0")" && pwd)
index=1
while [ "$index" -le "$attempts" ]; do
  echo "=== attempt $index at $(date '+%H:%M:%S') ===" >> "$capture/h1-dev-attempts.txt"
  if NX_DAEMON=false sh "$here/📜️h1-dev-boot.sh" "$capture" "$port" "$waits" >> "$capture/h1-dev-attempts.txt" 2>&1; then
    echo "attempt $index reached /readyz twice" >> "$capture/h1-dev-attempts.txt"
    cp "$capture/h1-dev-first.txt" "$capture/h1-dev-first-green.txt"
    cp "$capture/h1-dev-restart.txt" "$capture/h1-dev-restart-green.txt"
    exit 0
  fi
  grep -E "^error(\[|:)" "$capture/h1-dev-first.txt" | sed 's/\x1b\[[0-9;]*m//g' | head -3 >> "$capture/h1-dev-attempts.txt"
  index=$((index + 1))
  sleep 20
done
echo "no attempt reached /readyz" >> "$capture/h1-dev-attempts.txt"
exit 1
