#!/bin/zsh
# 🧪️ S18 overlay proof of the named-layout Rust twin (window-3 prepared patch): ONE overlay build via the `overlay` fleet mutex,
# inside the S18 scratch overlay with PRIVATE build/target dirs (never the shared build-dir), nice 15, incremental off.
overlay="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s18-overlay"
out="/Users/ueli/Documents/semio/.tmp-ticket/wp-s18/generated"
export CARGO_INCREMENTAL=0 RUST_MIN_STACK=33554432 NX_DAEMON=false
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s18-build"
export CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s18-target"
run() {
  local name="$1"; shift
  echo "START $name $(date '+%H:%M:%S')"
  ( cd "$overlay" && nice -n 15 cargo "$@" ) > "$out/s18-14b-ov-$name.txt" 2>&1
  echo "END $name rc=$? $(date '+%H:%M:%S') :: $(/usr/bin/grep -E '^test result|^error(\[|:)' "$out/s18-14b-ov-$name.txt" | sort | uniq -c | tr '\n' ' ' | cut -c1-600)"
}
run config test -p semio-framework-os-config --lib --no-fail-fast -- --test-threads 4
run renderer test -p semio-framework-os-renderer-wgpu --lib --no-fail-fast -- --test-threads 4 named_layout preference_log seeded_snapshot prefs
echo "S18-OVERLAY DONE $(date '+%H:%M:%S')"
