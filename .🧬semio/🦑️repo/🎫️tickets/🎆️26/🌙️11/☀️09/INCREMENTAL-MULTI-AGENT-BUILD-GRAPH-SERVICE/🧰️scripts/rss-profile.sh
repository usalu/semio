#!/usr/bin/env bash
# usage: rss-profile.sh <label> [stop]  runs the in-process planner harness under a 3 GiB cgroup cap; env snapshot of a live planner in 🗑️generated/rss-profile/env.txt
set -u
T="$(cd "$(dirname "$0")/.." && pwd)"; G="$T/🗑️generated/rss-profile"; mkdir -p "$G"
BUN="${BUN:-/home/deck/.cache/semio/runtimes/bun/c678040f14fe0440eb839d37cbd0ce4c051a32da72806ac97de6a6aab6bf728f/bun}"
F="$(ls -S /home/deck/semio/.cache/b/g/m/generations/x/*.facts.br | head -1)"
cd /home/deck/semio
env=(); while IFS= read -r line; do [ -n "$line" ] && env+=("$line"); done <"$G/env.txt"
systemd-run --user --scope --quiet -p MemoryMax="${CAP:-3G}" -p MemorySwapMax=0 env "${env[@]}" BUN_JSC_forceRAMSize=${FORCE:-2147483648} RSS_FACTS="$F" timeout 3000 "$BUN" ${CPU_PROF:+--cpu-prof-md --cpu-prof-dir "$G" --cpu-prof-name "$1.cpu.md"} ${HEAP_PROF:+--heap-prof-md --heap-prof-dir "$G" --heap-prof-name "$1.heap.md" --heap-prof-interval ${HEAP_PROF}} "$T/🧰️scripts/rss-profile.ts" "$G/$1.json" "${2:-all}" >"$G/$1.log" 2>&1
echo "exit $?" >>"$G/$1.log"
