#!/bin/sh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/SOURCING-END-TO-END"; OUT="$T/🗑️generated"
R=/Users/ueli/Documents/semio
PATH="$(ls -d $R/.cache/b/t/m/bun/*/ | head -1):$PATH"; export PATH
export CARGO_TARGET_DIR=/Users/ueli/t/src-curation CARGO_BUILD_BUILD_DIR=/Users/ueli/t/src-curation-b CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER= CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 SEMIO_CURATION_IO_OUT="$OUT/curation-io-out"
mkdir -p "$SEMIO_CURATION_IO_OUT"
cd "$R/✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation" || exit 1
cargo test ${CARGO_ARGS:---workspace --no-fail-fast} -j 4 -- --test-threads=1 > "$OUT/curation-native-test${SUFFIX}.txt" 2>&1
echo "curation exit $?" > "$OUT/curation-native-driver${SUFFIX}.log"
