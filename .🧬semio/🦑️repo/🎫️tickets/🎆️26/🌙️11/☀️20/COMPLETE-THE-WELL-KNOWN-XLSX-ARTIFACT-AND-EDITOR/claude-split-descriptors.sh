#!/bin/zsh
# [DEBUG] native emission of the committed semantic descriptor pairs of stdio, stdio-semio and stdio-bim (the descriptor_is_fresh test writes them before it compares)
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; N=$T/🗑️generated/claude-split/native-test
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$N/target CARGO_BUILD_BUILD_DIR=$N/build
export CARGO_BUILD_JOBS=2 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0
export SEMIO_DESCRIBE_EMIT_DIR=$T/🗑️generated/claude-split/descriptors
mkdir -p $SEMIO_DESCRIBE_EMIT_DIR
cd /Users/ueli/Documents/semio/🌎️hub || exit 1
for p in semio-hub-stdio-semio-component semio-hub-stdio-component; do
  cargo test -j 2 -p $p --lib descriptor_is_fresh -- --test-threads=1 || echo "[DEBUG] $p exit $? (stale descriptor expected before emission)"
done
