#!/bin/bash
# g8: build the plugin-host lib test binary once, keep a private copy (external sweeps wipe the shared build-dir),
# then run it three times back to back, restaging W1's current-tree release components before each run.
cd /Users/ueli/Documents/semio
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-g8
PKG="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust"
SRC="/Users/ueli/Documents/semio/.🧬semio/🌐hub/w1-catalog-a/trusted-catalog/build-31ee7a231be96f5440b5cec82abdbcbb"
DST="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target-g8/wasm32-wasip2/wasm-release"
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=$W/target cargo test -p semio-framework-plugin-host --lib --no-run --message-format=json > $W/generated/host-build.json 2> $W/generated/host-build.txt
echo "G8-BUILD-EXIT $?" >> $W/generated/host-build.txt
BIN=$(grep '"profile":{[^}]*"test":true' $W/generated/host-build.json | grep -o '"executable":"[^"]*"' | tail -1 | sed 's/"executable":"//;s/"$//')
cp "$BIN" $W/target/host-lib-bin || exit 1
for run in 1 2 3; do
  mkdir -p "$DST"; for p in stdio note gis; do cp "$SRC/$p-target/wasm32-wasip2/wasm-release/semio_s_plugin_$p.wasm" "$DST/"; done
  (cd "$PKG" && CARGO_MANIFEST_DIR="$PKG" RUST_MIN_STACK=33554432 $W/target/host-lib-bin > $W/generated/host-run-$run.txt 2>&1; echo "G8-RUN-EXIT $?" >> $W/generated/host-run-$run.txt)
done
