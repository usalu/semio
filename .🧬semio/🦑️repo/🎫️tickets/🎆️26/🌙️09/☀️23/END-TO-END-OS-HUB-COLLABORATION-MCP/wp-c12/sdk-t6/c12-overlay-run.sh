#!/bin/zsh
# 🧪️ C12: one command in a C12 APFS overlay under ONE overlay-lane hold (private build-dir, load gate beside the wasm lane, disk
# guard; preamble rules 23/25/26). usage: zsh c12-overlay-run.sh <capture> <overlay dir> <shell command>
OUT="$1"; OV="$2"; CMD="$3"
T=/Users/ueli/Documents/semio/.tmp-ticket/wp-t14
cd "$OV" || exit 1
echo "QUEUED $(date '+%F %T') $CMD" > "$OUT"
[ -d "$OV/.c12-build/debug" ] || python3 $T/overlay-build-seed.py "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b/debug" "$OV/.c12-build/debug" >> "$OUT" 2>&1
export CARGO_BUILD_BUILD_DIR="$OV/.c12-build" CARGO_TARGET_DIR="$OV/.c12-target" CARGO_INCREMENTAL=0 NX_DAEMON=false RUST_MIN_STACK=134217728 CARGO_NET_OFFLINE=true
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay c12 -- zsh -c '
  while [ -d /tmp/semio-wasm-build.lock ] && [ "$(sysctl -n vm.loadavg | awk "{print int(\$2)}")" -ge 16 ]; do sleep 60; done
  free=$(df -g / | awk "NR==2 {print \$4}")
  echo "=== lane $(date "+%T") load=$(sysctl -n vm.loadavg) free=${free}GiB"
  [ "$free" -ge 30 ] || { echo "DISK GUARD: ${free} GiB free < 30, not building"; exit 3; }
  nice -n 15 zsh -c "$0"; echo "RUN rc=$? $(date "+%T")"
' "$CMD" >> "$OUT" 2>&1
echo "EXIT rc=$? $(date '+%F %T')" >> "$OUT"
