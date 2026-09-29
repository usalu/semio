#!/bin/zsh
# 🧪️ RS1 overlay cargo: one cargo command inside the RS1 APFS-clone overlay with PRIVATE build/target dirs inside the overlay
# (never the shared build-dir), through the fleet mutex (rules 3/23/25/28). The build-dir is seeded once with registry units
# only (wp-t14/overlay-build-seed.py). usage: rs1-cargo.sh <lane> <capture-name> <cargo args…>
lane="$1"; name="$2"; shift 2
HUB="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
OV="$HUB/s15-rs1-overlay"
LOGS="$HUB/s15-rs1-logs"; mkdir -p "$LOGS"
capture="$LOGS/$name.txt"
export CARGO_BUILD_BUILD_DIR="$OV/.rs1-build" CARGO_TARGET_DIR="$OV/.rs1-target" CARGO_INCREMENTAL=0 NX_DAEMON=false RUST_MIN_STACK=33554432 CARGO_NET_OFFLINE=true
export CARGO_TARGET_WASM32_WASIP2_RUNNER="wasmtime run --dir=."
{ echo "QUEUED $(date '+%F %T') lane=$lane cargo $*"; zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh "$lane" rs1 -- zsh -c '
  echo "START $(date "+%T") load=$(sysctl -n vm.loadavg) free=$(df -g / | awk "NR==2 {print \$4}")GiB"
  [ -d "$CARGO_BUILD_BUILD_DIR/debug" ] || python3 /Users/ueli/Documents/semio/.tmp-ticket/wp-t14/overlay-build-seed.py "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b/debug" "$CARGO_BUILD_BUILD_DIR/debug"
  cd "'"$OV"'" || exit 6
  /usr/bin/time -l nice -n 15 cargo "$@"; echo "EXIT $? $(date "+%T")"' rs1 "$@"; } > "$capture" 2>&1
tail -3 "$capture"
