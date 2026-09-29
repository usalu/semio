#!/bin/zsh
# 🧪️ C12 T6 overlay proof of `c12-sdk-composition-patch.py` (applied ONLY in the APFS overlay `.🧬semio/🌐hub/s14-c12-overlay`), ONE
# overlay-lane hold (a `cargo check --lib --tests` of SDK + writer gates the runs), PRIVATE build-dir seeded with registry units only (preamble rules 23/25/26): writer `--lib` (the 6 T6 laws + the
# 10 000-keystroke burst), trinity jack `--lib --features component-app-assembly` (its editor laws, incl. the ledger twin, are feature-gated), SDK `--lib` narrowed to child/maintenance/composition laws.
# usage: zsh t6-overlay-proof.sh <capture>
OUT="$1"
OV="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-c12-overlay"
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
  nice -n 15 cargo check -p semio-framework-plugin -p semio-s-artifact-writer-writer --lib --tests; rc=$?; echo "CHECK rc=$rc $(date "+%T")"
  [ "$rc" -eq 0 ] || exit 4
  nice -n 15 cargo nextest run --profile long --no-fail-fast -p semio-s-artifact-writer-writer --lib; echo "WRITER rc=$? $(date "+%T")"
  nice -n 15 cargo nextest run --profile long --no-fail-fast -p semio-s-artifact-trinity-jack --lib --features component-app-assembly; echo "JACK rc=$? $(date "+%T")"
  nice -n 15 cargo nextest run --profile long --no-fail-fast -p semio-framework-plugin --lib -E "test(/child|maintenance|composition|retire|follow|member|backbone|typing/)"; echo "SDK rc=$? $(date "+%T")"
' >> "$OUT" 2>&1
echo "EXIT rc=$? $(date '+%F %T')" >> "$OUT"
