#!/bin/zsh
# 🧪️ SH2 overlay runs (session 14 rule 3 / session 13 rule 37): ONE overlay build fleet-wide via the `overlay` fleet mutex, inside the
# scratch overlay with PRIVATE build/target dirs (never the shared build-dir), nice 15, incremental off.
overlay="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-sh1-overlay"
out="/Users/ueli/Documents/semio/.tmp-ticket/wp-sh2/generated"
tag="${SH2_TAG:-ov}"
export CARGO_INCREMENTAL=0 RUST_MIN_STACK=33554432
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-sh1-build"
export CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-sh1-target"
run() {
  local name="$1"; shift
  echo "START $name $(date '+%H:%M:%S')"
  ( cd "$overlay" && zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay sh2 -- nice -n 15 cargo "$@" ) > "$out/$tag-$name.txt" 2>&1
  echo "END $name rc=$? $(date '+%H:%M:%S') :: $(/usr/bin/grep -E '^test result|^error(\[|:)' "$out/$tag-$name.txt" | sort | uniq -c | tr '\n' ' ' | cut -c1-600)"
}
jobs=("$@")
for job in $jobs; do
  case "$job" in
    check) run check check -p semio-s-artifact-space-home -p semio-s-plugin-space --features semio-s-artifact-space-home/component-app-assembly --lib --tests --message-format short ;;
    check-wasm) run check-wasm check -p semio-s-plugin-space --target wasm32-wasip2 --lib --message-format short ;;
    home) run home test -p semio-s-artifact-space-home --lib --no-fail-fast -- --test-threads 4 ;;
    home-feature) run home-feature test -p semio-s-artifact-space-home --features component-app-assembly --lib --no-fail-fast -- --test-threads 4 ;;
    space) run space test -p semio-s-artifact-space-space --features component-app-assembly --lib --no-fail-fast -- --test-threads 4 ;;
    plugin) run plugin test -p semio-s-plugin-space --lib --no-fail-fast -- --test-threads 4 ;;
    kernel) run kernel test -p semio-framework-os-kernel --features sync,ureq --lib --no-fail-fast -- --test-threads 4 ;;
  esac
done
echo "SH2-OVERLAY DONE $(date '+%H:%M:%S')"
