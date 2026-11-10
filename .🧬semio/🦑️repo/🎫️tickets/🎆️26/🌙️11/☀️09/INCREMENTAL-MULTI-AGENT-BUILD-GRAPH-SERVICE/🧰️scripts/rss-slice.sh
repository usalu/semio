#!/usr/bin/env bash
# usage: rss-service.sh <label>   in-process host-service graph load (loadResidentGraphV1 path) under a cgroup cap (CAP, default 5G); env of a live planner in 🗑️generated/rss-profile/env.txt
T="$(cd "$(dirname "$0")/.." && pwd)"; G="$T/🗑️generated/rss-profile"; BUN="${BUN:-/home/deck/.cache/semio/runtimes/bun/c678040f14fe0440eb839d37cbd0ce4c051a32da72806ac97de6a6aab6bf728f/bun}"
cd /home/deck/semio; env=(); while IFS= read -r l; do [ -n "$l" ] && env+=("$l"); done <"$G/env.txt"
systemd-run --user --scope --quiet -p MemoryMax="${CAP:-5G}" -p MemorySwapMax=0 env "${env[@]}" ${FORCE:+BUN_JSC_forceRAMSize=$FORCE} timeout 1500 "$BUN" "$T/🧰️scripts/rss-slice.ts" "$G/$1.json" ${POLICY:-fresh} >"$G/$1.log" 2>&1; echo "exit $?" >>"$G/$1.log"
