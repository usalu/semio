#!/bin/zsh
# [DEBUG] native test run of the live cleanup pump stall verdict (close-budget laws of the plugin runtime)
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; N=$T/🗑️generated/claude-cleanup-pump
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$N/target CARGO_BUILD_BUILD_DIR=$N/build
export CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0
cd /Users/ueli/Documents/semio || exit 1
code=0
cargo test -j 2 -p semio-plugin-tests-01-2 --lib -- runtime_close_budget_tests > $N/test.log 2>&1 || code=$?
cargo test -j 2 -p semio-plugin-runtime-instances --lib -- stall_laws >> $N/test.log 2>&1 || code=$?
cargo test -j 2 -p semio-plugin-document-media --lib -- worker_idle_laws >> $N/test.log 2>&1 || code=$?
echo "[DEBUG] tests exit $code" >> $N/test.log
exit $code
