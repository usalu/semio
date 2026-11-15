#!/bin/bash
# Usage: run.sh <n>  — observer run of the native bootstrap on the live tree under a declared 16 GB host.
set -u
n="$1"
N="/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES"
G="$N/n5-logs"; log="$G/n5-run-$n.log"; mem="$G/n5-run-$n.mem.txt"; done_file="$G/n5-run-$n.done"
worktree=/Users/Shared/semio
export SEMIO_LANGUAGE=en SEMIO_WORKSPACE_ROOT="$worktree" SEMIO_FLEET_GROUP=g1 SEMIO_FLEET_NFS_HOST=192.168.178.70 SEMIO_FLEET_NETWORK=192.168.178.0/24
export SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=16384
unset SEMIO_REMOTE_CACHE_ENDPOINT SEMIO_REMOTE_CACHE_ACCESS SEMIO_REMOTE_CACHE_INSTANCE SEMIO_REMOTE_CACHE_TOKEN SEMIO_FLEET_CACHE_ENDPOINT
export PATH="$HOME/.local/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin"
rm -f "$done_file"
cd "$worktree" || exit 1
( while :; do { date -u +"== %H:%M:%SZ"; ps -axo rss,command | sort -nr | head -3 | cut -c1-200; memory_pressure | /usr/bin/grep 'free percentage'; sysctl -n vm.swapusage; uptime; } >> "$mem" 2>&1; sleep 30; done ) &
sampler=$!
trap 'kill $sampler 2>/dev/null' EXIT
echo "start $(date -u +%FT%TZ) df=$(df -g /Users/Shared | tail -1 | awk '{print $4}')GiB" > "$log"
( if [ "${2:-}" = diag ]; then export PATH="$HOME/.local/bin:$PATH"; cd "$worktree"; echo "[n5] repo dependencies sync --all"; repo dependencies sync --all; r=$?; echo "[n5] sync rc=$r"; echo "[n5] repo run workspace:setup"; repo run workspace:setup; else bash "$worktree/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh" setup; fi ) 2>&1 | perl -MPOSIX -ne 'BEGIN{$|=1} print strftime("%H:%M:%SZ ",gmtime), $_' >> "$log"
rc=${PIPESTATUS[0]}
echo "end $(date -u +%FT%TZ)" >> "$log"
echo "native_rc=$rc" >> "$log"
echo "$rc" > "$done_file"
