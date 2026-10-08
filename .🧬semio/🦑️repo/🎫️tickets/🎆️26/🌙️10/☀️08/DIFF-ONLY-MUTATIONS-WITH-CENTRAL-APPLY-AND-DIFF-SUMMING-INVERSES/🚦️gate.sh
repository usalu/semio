#!/usr/bin/env bash
# 🚦️ Build gate: blocks until fewer than MAX_RUSTC rustc processes run and swap has headroom, then holds one of 4 slots.
# Usage: "$T/🚦️gate.sh" <label> -- <command...>
set -u
label="$1"; shift; [ "${1:-}" = "--" ] && shift
dir="$(cd "$(dirname "$0")" && pwd)/🗑️generated/gate"; mkdir -p "$dir"
max_rustc="${MAX_RUSTC:-10}"
while :; do
  for slot in 1 2 3 4; do
    if mkdir "$dir/slot-$slot" 2>/dev/null; then
      if [ "$(pgrep -x rustc | wc -l | tr -d ' ')" -lt "$max_rustc" ]; then
        echo "$label $$ $(date +%T)" > "$dir/slot-$slot/owner"
        trap 'rm -rf "$dir/slot-'"$slot"'"' EXIT
        CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-3}" "$@"; exit $?
      fi
      rm -rf "$dir/slot-$slot"
    elif [ -f "$dir/slot-$slot/owner" ] && ! kill -0 "$(awk '{print $2}' "$dir/slot-$slot/owner")" 2>/dev/null; then
      rm -rf "$dir/slot-$slot"
    fi
  done
  sleep 15
done
