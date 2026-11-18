#!/bin/bash
# Refreshes the staged copy from the live tree, then runs the switch from the copy.
set -euo pipefail
HERE="$HOME/g1-server-switch"
T="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🗺️topology"
S="/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/🧰️scripts/1114-g1-server-canonical"
[ -d "$T" ] && [ -f "$S/switch.sh" ] || { echo "ERROR: the live tree is not at /Users/Shared/semio (already switched?)"; exit 1; }
mkdir -p "$HERE"
rsync -a --delete "$T/" "$HERE/topology/"
cp -p "$S/switch.sh" "$S/rebind-check.ts" "$HERE/"
exec bash "$HERE/switch.sh" "$@"
