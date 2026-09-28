#!/bin/zsh
# 🦀️ ST2 native lane job: the host registry law that reads the corrected `🗄️artifact-kind-formats.json` fixture (host
# module is gated behind `os-host-full`).
# usage: zsh st2-native-host.zsh <capture>   (detached)
setopt no_bg_nice
capture="${1:A}"
export CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-st2/target NX_DAEMON=false
cd /Users/ueli/Documents/semio || exit 1
{
  echo "[st2-native] QUEUED $(date '+%F %T') pid $$"
  FLEET_TICKET_STAMP=20260928000003 zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native st2 -- zsh -c 'echo "[st2-native] HOLD $(date "+%F %T")"; nice -n 15 cargo test -p semio-framework-os --features os-host-full --lib --no-fail-fast --message-format=short owned_artifact_kind_formats; echo "[st2-native] host kind-formats rc=$? $(date "+%F %T")"'
  echo "[st2-native] END rc=$? $(date '+%F %T')"
} > "$capture" 2>&1
