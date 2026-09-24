#!/bin/zsh
# WAL-writer fence contract on live servers (the `wal-writer-fence-live` verb), on the H9 private target.
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h9/target RUST_MIN_STACK=268435456
export SEMIO_TEST_ARTIFACT_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/END-TO-END-OS-HUB-COLLABORATION-MCP/wp-h9/generated/test-artifacts-fence
mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
echo "=== start $(date +%T)"
cargo test -p semio-framework-os-kernel-db --features sqlite,postgres,neo4j --lib db_storage::writer::fence_conformance -- --include-ignored --test-threads=1 2>&1 | /usr/bin/grep -E "^test |test result|panicked|^error" 
echo "EXIT ${pipestatus[1]}"
echo "=== done $(date +%T)"
