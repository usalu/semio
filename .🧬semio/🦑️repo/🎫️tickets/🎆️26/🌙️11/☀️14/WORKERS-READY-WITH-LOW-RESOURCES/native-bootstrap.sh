#!/bin/bash
# 🛬 Runs the native bootstrap of the fleet worktree once as the worker account and writes the completion marker only after exit 0. Usage: native-bootstrap.sh <log>
set -u
case "$(uname -s)" in Darwin) worktree="$HOME/Documents/semio";; *) worktree="$HOME/semio";; esac
marker="$worktree/.cache/f/native-bootstrap.done"
export SEMIO_LANGUAGE=en SEMIO_WORKSPACE_ROOT="$worktree" SEMIO_FLEET_GROUP=g1 SEMIO_FLEET_NFS_HOST=192.168.178.70 SEMIO_FLEET_NETWORK=192.168.178.0/24
unset SEMIO_REMOTE_CACHE_ENDPOINT SEMIO_REMOTE_CACHE_ACCESS SEMIO_REMOTE_CACHE_INSTANCE SEMIO_REMOTE_CACHE_TOKEN SEMIO_FLEET_CACHE_ENDPOINT
export PATH="$HOME/.local/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin"
rm -f "$marker"
cd "$worktree" || exit 1
date -u +"start %Y-%m-%dT%H:%M:%SZ" > "$1"
bash "$worktree/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh" setup 2>&1 | tee "$worktree/.cache/f/native-bootstrap.log" >> "$1"
status=${PIPESTATUS[0]}
date -u +"end %Y-%m-%dT%H:%M:%SZ" >> "$1"
if [ "$status" = 0 ] && [ -x "$HOME/.local/bin/repo" ]; then date -u +%Y-%m-%dT%H:%M:%SZ > "$marker"; echo "native_rc=0 marker written" >> "$1"; else echo "native_rc=$status" >> "$1"; fi
