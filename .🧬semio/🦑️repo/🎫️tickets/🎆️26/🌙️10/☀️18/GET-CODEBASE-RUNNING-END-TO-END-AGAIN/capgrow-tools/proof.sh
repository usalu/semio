#!/bin/sh
# 🧪️ capgrow product proof v3: a holder pins real memory (pin.py) under a FRESH gate directory shared with the build (no class history: not forecast quiet, fresh admission path); the pin is released 5 s after the build printed its admission, so the build outlives it by its whole remaining time; the grower then needs two admitted heartbeats (15 s apart): `grown from` appears 15-30 s after the release.
# usage: sh proof.sh [label] [target] [extra repo-run args...]   default: g41-grow @semio-tech/print-rs:test --skip-cache
C="$(cd "$(dirname "$0")/.." && pwd)"; cd /Users/ueli/Documents/semio || exit 2
label=${1:-g41-grow}; target=${2:-@semio-tech/print-rs:test}; shift 2 2>/dev/null; [ $# -eq 0 ] && set -- --skip-cache
gate=$(mktemp -d "$C/🗑️generated/capgrow/gate.XXXXXX"); export SEMIO_HOST_CAPACITY_DIRECTORY="$gate"; release="$gate/release"; log="$C/🗑️generated/actenv/$label.log"
free_mb=$(vm_stat | awk '/Pages free|Pages inactive|Pages speculative/ {s+=$NF} END {printf "%d", s*16384/1048576}')
pin=$((free_mb - 4096 - 5000)); [ $pin -lt 2000 ] && { echo "[capgrow-proof] only $free_mb MB free: not enough to pin"; exit 2; }
echo "[capgrow-proof] gate $gate, pinning $pin MB (free $free_mb MB) until 5 s after the build is admitted"
env -u CLAUDECODE bun repo capacity run --memory=$pin -- python3 -I "$C/capgrow-tools/pin.py" $pin 600 "$release" > "$C/🗑️generated/capgrow/$label-holder.log" 2>&1 &
holder=$!
for _ in $(seq 1 120); do grep -q "DEBUG\] pinned" "$C/🗑️generated/capgrow/$label-holder.log" 2>/dev/null && break; sleep 1; done
rm -f "$log"; python3 "$C/actenv-tools/product_skip.py" "$label" "$target" "$@" & build=$!
for _ in $(seq 1 300); do grep -q "^\[capacity\] admitted\|^\[capacity\] .* zugelassen" "$log" 2>/dev/null && break; sleep 1; done
sleep 5; touch "$release"; echo "[capgrow-proof] pin released at $(date +%H:%M:%S)"
wait $build; wait $holder
grep -h "\[capacity\]\|\[repo cargo\]\|\[DEBUG\] pinned\|\[DEBUG\] released\|Elapsed time" "$C/🗑️generated/capgrow/$label-holder.log" "$log" | cut -c1-260
