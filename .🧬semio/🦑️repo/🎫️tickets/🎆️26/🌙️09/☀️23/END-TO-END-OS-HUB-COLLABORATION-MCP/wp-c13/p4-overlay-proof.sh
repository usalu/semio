#!/bin/zsh
# 🧪️ C13 (session 15): P4 v2 (`p4-ephemeral-on-continuation.py`, T6 row 18) proven in the C13 overlay under ONE overlay-lane hold
# (private build-dir seeded with registry units only, disk guard 30 GiB, preamble rules 3/23/25/28). The SDK file is
# `semio-framework-plugin`'s `component` module. Outside the lane the overlay's two files are reset to their pre-P4 bytes.
#   M  mutant = P4's law with the pre-P4 SDK: the SDK crate's whole `--lib` suite ×3 — the law must FAIL every run; every other
#      red is the tree's baseline (a red in only some runs is a flake)
#   P  P4 applied: the same suite ×3 — the law must PASS every run; reds must stay inside M's
#   W  the wasm32-wasip2 `--lib` check of the SDK crate with P4
# usage: zsh p4-overlay-proof.sh <tag>   (capture `.🧬semio/🌐hub/s14-c13-runs/p4-overlay-<tag>.txt`)
OV="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-c13-overlay"
OUT="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-c13-runs/p4-overlay-$1.txt"
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-c13
BK="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-c13-w3-backup/c13-p4-s14-c13-overlay"
SDK="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
LAWS="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"
cd "$OV" || exit 1
echo "QUEUED $(date '+%F %T')" > "$OUT"
cp "$BK/before/$SDK" "$OV/$SDK" && cp "$BK/before/$LAWS" "$OV/$LAWS" || { echo "RESET FAILED" >> "$OUT"; exit 2; }
python3 "$W/p4-ephemeral-on-continuation.py" --root "$OV" --write >> "$OUT" 2>&1 || { echo "WRITE FAILED" >> "$OUT"; exit 2; }
cp "$BK/before/$SDK" "$OV/$SDK" && touch "$OV/$SDK" "$OV/$LAWS"
[ -d "$OV/.c13-build/debug" ] || python3 /Users/ueli/Documents/semio/.tmp-ticket/wp-t14/overlay-build-seed.py "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b/debug" "$OV/.c13-build/debug" >> "$OUT" 2>&1
export CARGO_BUILD_BUILD_DIR="$OV/.c13-build" CARGO_TARGET_DIR="$OV/.c13-target" CARGO_INCREMENTAL=0 NX_DAEMON=false RUST_MIN_STACK=134217728 CARGO_NET_OFFLINE=true
export C13_OV="$OV" C13_W="$W" C13_SDK="$SDK"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay c13 -- zsh -c '
  free=$(df -g / | awk "NR==2 {print \$4}")
  echo "=== lane $(date "+%T") load=$(sysctl -n vm.loadavg) free=${free}GiB"
  [ "$free" -ge 30 ] || { echo "DISK GUARD: ${free} GiB free < 30, not building"; exit 3; }
  for run in 1 2 3; do
    echo "=== M$run mutant (P4 law, pre-P4 SDK) $(date "+%T")"
    nice -n 15 cargo test -p semio-framework-plugin --lib --no-fail-fast; echo "M$run rc=$? $(date "+%T")"
  done
  python3 "$C13_W/p4-ephemeral-on-continuation.py" --root "$C13_OV" --write | tail -1 && touch "$C13_OV/$C13_SDK"
  for run in 1 2 3; do
    echo "=== P$run P4 v2 $(date "+%T")"
    nice -n 15 cargo test -p semio-framework-plugin --lib --no-fail-fast; echo "P$run rc=$? $(date "+%T")"
  done
  echo "=== W wasm32-wasip2 check $(date "+%T")"
  nice -n 15 cargo check -p semio-framework-plugin --lib --target wasm32-wasip2; echo "W rc=$? $(date "+%T")"
' >> "$OUT" 2>&1
echo "EXIT rc=$? $(date '+%F %T')" >> "$OUT"
