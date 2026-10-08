#!/usr/bin/env bash
# 🚦️ Build gate (Windows/Git Bash): waits for a free slot and fewer than MAX_RUSTC rustc processes, then runs the command.
# Usage: "$T/🚦️gate.sh" <label> -- <command...>
set -u
label="$1"; shift; [ "${1:-}" = "--" ] && shift
dir="$(cd "$(dirname "$0")" && pwd)/🗑️generated/gate"; mkdir -p "$dir"
max_rustc="${MAX_RUSTC:-14}"
slots="${GATE_SLOTS:-4}"
count_rustc() { tasklist //FI "IMAGENAME eq rustc.exe" 2>/dev/null | grep -ci "rustc.exe"; }
while :; do
  for slot in $(seq 1 "$slots"); do
    if mkdir "$dir/slot-$slot" 2>/dev/null; then
      if [ "$(count_rustc)" -lt "$max_rustc" ]; then
        echo "$label $$ $(date +%T)" > "$dir/slot-$slot/owner"
        trap 'rm -rf "$dir/slot-'"$slot"'"' EXIT
        cache="$(cd "$(dirname "$0")/../../../../../⚡️cache/cargo" && pwd)"
        export RUSTC_WRAPPER="" CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-4}" \
          CARGO_UNSTABLE_BUILD_DIR_NEW_LAYOUT=false CARGO_UNSTABLE_FINE_GRAIN_LOCKING=false \
          CARGO_BUILD_BUILD_DIR="$cache/build-bim-$slot" CARGO_TARGET_DIR="$cache/target-bim-$slot"
        "$@"; exit $?
      fi
      rm -rf "$dir/slot-$slot"
    elif [ -f "$dir/slot-$slot/owner" ]; then
      owner="$(awk '{print $2}' "$dir/slot-$slot/owner")"
      if ! kill -0 "$owner" 2>/dev/null; then rm -rf "$dir/slot-$slot"; fi
    fi
  done
  sleep 10
done
