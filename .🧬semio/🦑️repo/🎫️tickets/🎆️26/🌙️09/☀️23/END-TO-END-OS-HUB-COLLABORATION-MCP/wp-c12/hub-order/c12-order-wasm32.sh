#!/bin/zsh
# 🧪️ C12: the hub-order set's wasm32 proof in overlay `s14-c12-overlay-order` under ONE wasm-lane hold (private build-dir, never the shared one;
# the browser `wasm_actor` is cfg(wasm32 ∧ ¬p2), invisible to every native check). Launch only after the chain releases the wasm lane
# (session-15 preamble; rule 28: this job holds no other lane). Only the browser actor is wasm-gated: the store, codec and
# host changes are target-independent and proven by the native overlay proof (`order-overlay-proof-4.txt`). usage: zsh c12-order-wasm32.sh <capture> [--dry-run]
OUT="$1"
OV="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-c12-overlay-order"
CMD='cargo check --keep-going --lib --message-format=short --target wasm32-unknown-unknown -p semio-framework-os-kernel --features sync; echo "WASM_WEB rc=$? $(date +%T)"'
if [ "$2" = "--dry-run" ]; then echo "overlay $OV"; echo "lane wasm c12"; echo "$CMD"; exit 0; fi
cd "$OV" || exit 1
echo "QUEUED $(date '+%F %T') $CMD" > "$OUT"
export CARGO_BUILD_BUILD_DIR="$OV/.c12-build" CARGO_TARGET_DIR="$OV/.c12-target" CARGO_INCREMENTAL=0 NX_DAEMON=false CARGO_NET_OFFLINE=true
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh wasm c12 -- zsh -c '
  free=$(df -g / | awk "NR==2 {print \$4}")
  echo "=== lane $(date "+%T") load=$(sysctl -n vm.loadavg) free=${free}GiB"
  [ "$free" -ge 30 ] || { echo "DISK GUARD: ${free} GiB free < 30, not building"; exit 3; }
  nice -n 15 zsh -c "$0"; echo "RUN rc=$? $(date "+%T")"
' "$CMD" >> "$OUT" 2>&1
echo "EXIT rc=$? $(date '+%F %T')" >> "$OUT"
