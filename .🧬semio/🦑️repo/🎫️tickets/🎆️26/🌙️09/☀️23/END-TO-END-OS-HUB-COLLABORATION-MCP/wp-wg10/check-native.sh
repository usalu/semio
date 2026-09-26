#!/bin/zsh
# WG10 s13: one native check of every crate WG10's landing touches (kernel with sync+ureq, plugin host, renderer), lib + tests.
# usage: zsh check-native.sh [crate-args…]  (default: the three crates); rule 26 build-dir.
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-wg10/target CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b
echo "START $(date '+%F %T')"
if [ $# -gt 0 ]; then
  nice -n 10 cargo check "$@" --lib --tests --keep-going --message-format short
else
  nice -n 10 cargo check -p semio-framework-os-kernel --features sync,ureq -p semio-framework-plugin-host -p semio-framework-os-renderer-wgpu --lib --tests --keep-going --message-format short
fi
echo "RC=$? END $(date '+%F %T')"
