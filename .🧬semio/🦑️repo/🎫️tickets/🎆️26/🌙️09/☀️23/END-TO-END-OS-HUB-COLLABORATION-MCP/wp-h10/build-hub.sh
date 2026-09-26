#!/bin/zsh
# 🏗️ H10: os-hub (default features) from this tree into the H10 private target, then a copy under .🧬semio/🌐hub/s12-h10-bin/<tag>.
# usage: build-hub.sh <tag> [extra cargo args…]
TAG=$1; shift
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h10/target RUST_MIN_STACK=268435456
echo "=== start $(date +%T) tag=$TAG"
nice -n 15 cargo build -p semio-hub --bin os-hub "$@" 2>&1 | /usr/bin/grep -E "^(error|warning)|Finished|-->" | tail -60
rc=${pipestatus[1]}
echo "EXIT $rc"
if [ "$rc" = 0 ]; then
  BIN="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s12-h10-bin/os-hub-$TAG"
  rm -f "$BIN"; cp "$CARGO_TARGET_DIR/debug/os-hub" "$BIN"; codesign -s - -f "$BIN" 2>&1
  ls -la "$BIN"; shasum -a 256 "$BIN"
fi
echo "=== done $(date +%T)"
