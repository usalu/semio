#!/bin/zsh
# 🧪️ S18 overlay proof of the named-layout Rust twin (window-3 prepared patch): ONE overlay build via the `overlay` fleet mutex,
# inside the S18 scratch overlay with PRIVATE build/target dirs (never the shared build-dir), nice 15, incremental off. A disk
# watchdog stops the running cargo when the volume drops below 20 GiB free (the chain must never starve).
overlay="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s18-overlay"
out="/Users/ueli/Documents/semio/.tmp-ticket/wp-s18/generated"
export CARGO_INCREMENTAL=0 RUST_MIN_STACK=33554432 NX_DAEMON=false
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s18-build"
export CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s18-target"
free_gib() { df -g /Users/ueli/Documents/semio | awk 'NR==2 {print $4}'; }
run() {
  local name="$1"; shift
  local capture="$out/${S18_OV_PREFIX:-s18-14b-ov}-$name.txt"
  echo "START $name $(date '+%H:%M:%S') free=$(free_gib)GiB"
  ( cd "$overlay" && exec nice -n 15 cargo "$@" ) > "$capture" 2>&1 &
  local pid=$!
  while kill -0 $pid 2>/dev/null; do
    if [ "$(free_gib)" -lt 20 ]; then
      echo "DISK-GUARD $name: free $(free_gib) GiB < 20 → stopping cargo $pid $(date '+%H:%M:%S')"
      pkill -TERM -P $pid 2>/dev/null; kill -TERM $pid 2>/dev/null
      break
    fi
    sleep 20
  done
  wait $pid
  echo "END $name rc=$? $(date '+%H:%M:%S') free=$(free_gib)GiB :: $(/usr/bin/grep -E '^test result|^error(\[|:)' "$capture" | sort | uniq -c | tr '\n' ' ' | cut -c1-600)"
}
run config test -p semio-framework-os-config --lib --no-fail-fast -- --test-threads 4
run renderer test -p semio-framework-os-renderer-wgpu --lib --no-fail-fast -- --test-threads 4 named_layout preference_log seeded_snapshot prefs
echo "S18-OVERLAY DONE $(date '+%H:%M:%S')"
