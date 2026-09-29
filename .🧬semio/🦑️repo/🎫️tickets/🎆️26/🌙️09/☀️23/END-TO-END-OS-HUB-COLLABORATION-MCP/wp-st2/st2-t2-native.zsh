#!/bin/zsh
# 🦀️ ST2 T2 laws (native lane, one job): stdio shipped_fleet + editor_catalog on the live tree after T2.
# usage: zsh st2-t2-native.zsh <capture>   (detached via w2-detach.py; captures live under .🧬semio/🌐hub/s14-st2-captures)
setopt no_bg_nice
capture="${1:A}"
export CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-st2-target NX_DAEMON=false
cd /Users/ueli/Documents/semio || exit 1
{
  echo "[st2-t2] QUEUED $(date '+%F %T') pid $$"
  zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native st2 -- zsh -c 'echo "[st2-t2] HOLD $(date "+%F %T") jobs=$CARGO_BUILD_JOBS"; nice -n 15 cargo test -p semio-s-plugin-stdio --test shipped_fleet --test editor_catalog --no-fail-fast --message-format=short; echo "[st2-t2] stdio laws rc=$? $(date "+%F %T")"'
  echo "[st2-t2] END rc=$? $(date '+%F %T')"
} > "$capture" 2>&1
