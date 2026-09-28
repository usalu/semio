#!/bin/zsh
# 🦀️ ST2 overlay lane step: queues one command in the fleet `overlay` lane and runs it inside the ST2 overlay
# (`s13-cx1-overlay`, synced by st2-overlay.py) with the overlay's PRIVATE build-dir and target-dir (never the shared
# build-dir), bounded by st2-hold.zsh's deadline. Launch detached: setopt no_bg_nice; nohup zsh st2-lane.zsh … & disown
# usage: zsh st2-lane.zsh <capture> <deadline-minutes> <command…>
setopt no_bg_nice
capture="${1:A}"; minutes="$2"; shift 2
H=/Users/ueli/Documents/semio/.🧬semio/🌐hub
export CARGO_BUILD_BUILD_DIR="$H/s13-cx1-build" CARGO_TARGET_DIR="$H/s13-cx1-target" CARGO_INCREMENTAL=0 CARGO_PROFILE_WASM_DEV_DEBUG=false RUSTC_WRAPPER="" NX_DAEMON=false
cd "$H/s13-cx1-overlay" || exit 1
{
  echo "[st2-lane] QUEUED $(date '+%F %T') pid $$: $*"
  zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay st2 -- zsh /Users/ueli/Documents/semio/.tmp-ticket/wp-st2/st2-hold.zsh "$minutes" "$@"
  echo "[st2-lane] END rc=$? $(date '+%F %T')"
} > "$capture" 2>&1
