#!/bin/zsh
# 🧪️ C12: scratch proof of the c12-splice set in the APFS overlay `.🧬semio/🌐hub/s14-c12-overlay` (set applied there), ONE overlay-lane
# hold, PRIVATE build-dir seeded with registry units only: ui (check), ui-scene (laws), writer (laws), then os tsc (TextEditor host).
# usage: zsh scratch-proof.sh <capture>
OUT="$1"
OV="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-c12-overlay"
T=/Users/ueli/Documents/semio/.tmp-ticket/wp-t14
cd "$OV" || exit 1
[ -d "$OV/.c12-build/debug" ] || python3 $T/overlay-build-seed.py "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b/debug" "$OV/.c12-build/debug" >> "$OUT" 2>&1
export CARGO_BUILD_BUILD_DIR="$OV/.c12-build" CARGO_TARGET_DIR="$OV/.c12-target" CARGO_INCREMENTAL=0 NX_DAEMON=false
echo "START $(date '+%F %T')" >> "$OUT"
# ⚖️ preamble rule 25(b): ONE heavy job machine-wide — the overlay lane is only taken while no native or wasm lane job runs.
while [ -d /tmp/semio-native-build.lock ] || [ -d /tmp/semio-wasm-build.lock ]; do sleep 60; done
echo "=== native + wasm lanes idle $(date '+%T')" >> "$OUT"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay c12 -- zsh -c '
  echo "=== lane $(date "+%T")"
  nice -n 15 cargo check -p semio-framework-ui --lib; echo "UI CHECK rc=$? $(date "+%T")"
  nice -n 15 cargo test -p semio-framework-ui-scene --lib --no-fail-fast; echo "SCENE TEST rc=$? $(date "+%T")"
  nice -n 15 cargo test -p semio-s-artifact-writer-writer --lib --no-fail-fast; echo "WRITER TEST rc=$? $(date "+%T")"
  cd "$0/🧰️framework/🛍️products/💻️os" && nice -n 15 bunx tsc --noEmit -p tsconfig.json; echo "OS TSC rc=$? $(date "+%T")"
' "$OV" >> "$OUT" 2>&1
echo "EXIT rc=$? $(date '+%F %T')" >> "$OUT"
