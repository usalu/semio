#!/bin/zsh
# WG10 s13: the renderer laws of WG10's landing (native AccessKit bridge, keyboard ring, directory door binary reads, chrome a11y).
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-wg10/target CARGO_BUILD_BUILD_DIR=${WG10_BUILD_DIR:-/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b}
echo "START $(date '+%F %T')"
nice -n 10 cargo test -p semio-framework-os-renderer-wgpu --lib --no-fail-fast -- --test-threads=1 native_accessibility native_keyboard_ring directory_door chrome_accessibility_dispatch
echo "RC=$? END $(date '+%F %T')"
