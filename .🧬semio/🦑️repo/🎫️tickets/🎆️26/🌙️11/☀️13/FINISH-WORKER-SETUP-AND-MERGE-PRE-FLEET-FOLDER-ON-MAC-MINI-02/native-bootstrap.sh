#!/bin/bash
# 🔩 Runs the native bootstrap of the fleet worktree as the worker account, the way step 9 of the provision script does, and records the completion marker only after it succeeded.
# Usage: [SEMIO_GRAPH_MEMORY_MB=<n>] native-bootstrap.sh <log>
set -u
worktree=/Users/ueli/Documents/semio
marker=$worktree/.cache/f/native-bootstrap.done
log=$worktree/.cache/f/native-bootstrap.log
export SEMIO_LANGUAGE=en SEMIO_WORKSPACE_ROOT=$worktree SEMIO_FLEET_GROUP=g1 SEMIO_FLEET_NFS_HOST=192.168.178.70 SEMIO_FLEET_NETWORK=192.168.178.0/24
export SEMIO_REMOTE_CACHE_ENDPOINT=http://192.168.178.70:6090 SEMIO_REMOTE_CACHE_ACCESS=read-write SEMIO_REMOTE_CACHE_INSTANCE=semio
export SEMIO_REMOTE_CACHE_TOKEN=$(sed -n "s/^export SEMIO_REMOTE_CACHE_TOKEN=//p" "$HOME/.zprofile" | tail -n 1 | tr -d "'\"")
export PATH="$HOME/.local/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin"
# SEMIO_GRAPH_MEMORY_MB (build guide, "Graph") caps the graph inference memory; pass it in the environment on a host that cannot spare the default.
rm -f "$marker"
cd "$worktree" || exit 1
bash "$worktree/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh" setup 2>&1 | tee "$log" > "$1"
status=${PIPESTATUS[0]}
if [ "$status" = 0 ] && [ -x "$HOME/.local/bin/repo" ]; then date -u +%Y-%m-%dT%H:%M:%SZ > "$marker"; echo "native_rc=0 marker written" >> "$1"; else echo "native_rc=$status" >> "$1"; fi
