#!/bin/zsh
# 🦀️ ST2 overlay cargo: runs one cargo command inside the ST2 overlay (CX1's overlay path, synced by st2-overlay.py) through
# the fleet `overlay` lane with the overlay's PRIVATE build-dir and target-dir (never the shared build-dir).
# usage: zsh st2-cargo.sh <capture> <cargo args…>
setopt no_bg_nice
capture="$1"; shift
H=/Users/ueli/Documents/semio/.🧬semio/🌐hub
export CARGO_BUILD_BUILD_DIR="$H/s13-cx1-build" CARGO_TARGET_DIR="$H/s13-cx1-target" CARGO_INCREMENTAL=0 CARGO_PROFILE_WASM_DEV_DEBUG=false RUSTC_WRAPPER=""
cd "$H/s13-cx1-overlay" || exit 1
{
  echo "[st2-cargo] START $(date '+%F %T') cargo $*"
  zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay st2 -- nice -n 15 cargo "$@"
  rc=$?
  echo "[st2-cargo] END rc=$rc $(date '+%F %T')"
} > "$capture" 2>&1
