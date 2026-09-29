#!/bin/zsh
# 🧪️ C12 T6 overlay proof of `c12-jack-query-document-patch.py` (applied ONLY in `.🧬semio/🌐hub/s14-c12-overlay-jack`), ONE overlay-lane
# hold, PRIVATE build-dir seeded with registry units only (preamble rules 23/25/26): `cargo check --lib --tests` of trinity jack (+ its
# app-assembly feature), the trinity mutation bridge and the trinity plugin gate the jack `--lib` laws.
# usage: zsh jack-overlay-proof.sh <capture>
OUT="$1"
OV="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-c12-overlay-jack"
T=/Users/ueli/Documents/semio/.tmp-ticket/wp-t14
cd "$OV" || exit 1
echo "QUEUED $(date '+%F %T')" > "$OUT"
[ -d "$OV/.c12-build/debug" ] || python3 $T/overlay-build-seed.py "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b/debug" "$OV/.c12-build/debug" >> "$OUT" 2>&1
export CARGO_BUILD_BUILD_DIR="$OV/.c12-build" CARGO_TARGET_DIR="$OV/.c12-target" CARGO_INCREMENTAL=0 NX_DAEMON=false RUST_MIN_STACK=134217728 CARGO_NET_OFFLINE=true
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay c12 -- zsh -c '
  while [ -d /tmp/semio-wasm-build.lock ] && [ "$(sysctl -n vm.loadavg | awk "{print int(\$2)}")" -ge 16 ]; do sleep 60; done
  free=$(df -g / | awk "NR==2 {print \$4}")
  echo "=== lane $(date "+%T") load=$(sysctl -n vm.loadavg) free=${free}GiB"
  [ "$free" -ge 30 ] || { echo "DISK GUARD: ${free} GiB free < 30, not building"; exit 3; }
  nice -n 15 cargo check -p semio-s-artifact-trinity-jack --lib --tests --features component-app-assembly; rc=$?; echo "CHECK jack rc=$rc $(date "+%T")"
  nice -n 15 cargo check -p semio-trinity-mutation-bridge -p semio-s-plugin-trinity --lib --tests; echo "CHECK trinity rc=$? $(date "+%T")"
  [ "$rc" -eq 0 ] || exit 4
  nice -n 15 cargo nextest run --profile long --no-fail-fast -p semio-s-artifact-trinity-jack --lib --features component-app-assembly; echo "JACK rc=$? $(date "+%T")"
' >> "$OUT" 2>&1
echo "EXIT rc=$? $(date '+%F %T')" >> "$OUT"
