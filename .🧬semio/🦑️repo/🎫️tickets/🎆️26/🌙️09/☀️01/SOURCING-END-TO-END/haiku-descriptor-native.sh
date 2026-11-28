#!/bin/sh
# 🧪 Native sequential cargo test runs for the descriptor freshness check (one build at a time, shared target dir).
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/SOURCING-END-TO-END"; OUT="$T/🗑️generated"
R=/Users/ueli/Documents/semio
export CARGO_TARGET_DIR=/Users/ueli/t/src-extensions CARGO_BUILD_BUILD_DIR=/Users/ueli/t/src-extensions-b CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER= CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 SEMIO_TEST_ARTIFACT_DIR="$OUT/artifacts"
mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
S="$R/✏️s/🔌️plugins/🪵️sourcing/🧩️extensions"
run() {
  crate=$1; dir=$2
  (cd "$dir" && cargo test -p "$crate" -j 4 -- --test-threads=1) > "$OUT/haiku-descriptor-$crate.log" 2>&1
  echo "$crate exit $?" >> "$OUT/haiku-descriptor-native-driver.log"
}
run semio-s-plugin-sourcing-beams-component "$S/🪵️beams/🧩️component/📦️packages/🦀️rust"
run semio-s-plugin-sourcing-slabs-component "$S/🧱️slabs/🧩️component/📦️packages/🦀️rust"
run semio-s-plugin-sourcing-windows-component "$S/🪟️windows/🧩️component/📦️packages/🦀️rust"
run semio-hub-sourcing-component "$R/🌎️hub/🧩️compositions/🪵️sourcing/🧩️component/📦️packages/🦀️rust"
echo finished >> "$OUT/haiku-descriptor-native-driver.log"
