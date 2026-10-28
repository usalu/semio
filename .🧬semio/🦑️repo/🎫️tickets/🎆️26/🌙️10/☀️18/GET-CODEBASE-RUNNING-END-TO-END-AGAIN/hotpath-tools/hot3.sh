#!/bin/sh
# 🔥 Hot no-op x N with a per-process RSS/CPU census beside each run (for the coordinator): sh hot3.sh <prefix> [target] [runs] [profile:on|off]
# Each run is gate-g's product.py (cleaned env, nice 10) plus treesample.py (read-only ps observer); the census lands in 🗑️generated/hotpath/<prefix>.jsonl, the logs in gate-g-tools/run/<prefix>N.log.
HERE=$(cd "$(dirname "$0")" && pwd); C=$(dirname "$HERE"); P=${1:?prefix}; T=${2:-@teaching/architecture-course:build}; N=${3:-3}; PROF=${4:-off}
mkdir -p "$C/🗑️generated/hotpath"
i=1
while [ "$i" -le "$N" ]; do
  python3 -I "$HERE/treesample.py" "$P$i" "$C/🗑️generated/hotpath/$P.jsonl" 900 > "$C/🗑️generated/hotpath/$P$i.tree.txt" 2>&1 &
  S=$!
  if [ "$PROF" = on ]; then SEMIO_REPO_PROFILE=json python3 "$C/gate-g-tools/product.py" "$P$i" "$T"; else python3 "$C/gate-g-tools/product.py" "$P$i" "$T"; fi
  wait "$S"; cat "$C/🗑️generated/hotpath/$P$i.tree.txt"
  i=$((i + 1))
done
