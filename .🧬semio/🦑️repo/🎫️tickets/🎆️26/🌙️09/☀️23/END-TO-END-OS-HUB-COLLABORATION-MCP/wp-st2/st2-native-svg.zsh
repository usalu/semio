#!/bin/zsh
# 🦀️ ST2 native lane job: the stale svg native-codec pin (peer 03:41 changed svg's snapshot `📡️.protocol.semio`; the
# definition + registry projection kept the old SHA-256 → every stdio/gis `plugin()` assembly refused) — stdio
# native_openable_provider laws (incl. projection == live receipts) and the hub consumer law that tripped on it.
# usage: zsh st2-native-svg.zsh <capture>   (detached; priority stamp: chain fix)
setopt no_bg_nice
capture="${1:A}"
export CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-st2/target NX_DAEMON=false
cd /Users/ueli/Documents/semio || exit 1
{
  echo "[st2-native] QUEUED $(date '+%F %T') pid $$"
  FLEET_TICKET_STAMP=20260928000001 zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native st2 -- zsh -c 'echo "[st2-native] HOLD $(date "+%F %T")"; nice -n 15 cargo test -p semio-hub --lib --no-fail-fast --message-format=short linked_consumer_descriptors_bind; echo "[st2-native] hub consumer rc=$? $(date "+%F %T")"; nice -n 15 cargo test -p semio-s-plugin-stdio --features full-artifact-catalog --test native_openable_provider --no-fail-fast --message-format=short; echo "[st2-native] stdio provider rc=$? $(date "+%F %T")"'
  echo "[st2-native] END rc=$? $(date '+%F %T')"
} > "$capture" 2>&1
