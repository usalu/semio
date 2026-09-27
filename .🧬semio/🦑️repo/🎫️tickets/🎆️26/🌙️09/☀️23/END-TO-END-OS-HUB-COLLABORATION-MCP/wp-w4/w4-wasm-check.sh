#!/bin/zsh
# 🧊️ W4: one wasm32-wasip2 `cargo check --lib` of the given crates in the chain's DEFAULT build-dir (warms the chain's guest-framework gate).
# usage (through the wasm lane): zsh 📜️fleet-mutex.sh wasm w4 -- zsh w4-wasm-check.sh <capture> <crate…>
setopt no_bg_nice
OUT="$1"; shift
cd /Users/ueli/Documents/semio || exit 1
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR CARGO_BUILD_BUILD_DIR
export CARGO_INCREMENTAL=0
args=(); for crate in "$@"; do args+=(-p "$crate"); done
s=$(date +%s)
echo "[w4-wasm-check] START ${(j: :)@} $(date '+%F %T')"
cargo check --lib --target wasm32-wasip2 $args --message-format short > "$OUT" 2>&1
rc=$?
echo "[w4-wasm-check] END rc=$rc wall=$(( $(date +%s) - s ))s $(date '+%F %T')"
exit $rc
