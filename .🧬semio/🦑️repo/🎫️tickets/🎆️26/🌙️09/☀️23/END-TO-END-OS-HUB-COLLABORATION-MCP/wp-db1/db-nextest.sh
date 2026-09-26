#!/bin/zsh
# DB1: full db lib suite under nextest (every law in its own process) on the DB1 private target.
cd /Users/ueli/Documents/semio
export CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-db1/target RUST_MIN_STACK=268435456
echo "=== nextest start $(date +%T)"
nice -n 10 cargo nextest run -p semio-framework-os-kernel-db --features sqlite,postgres,neo4j --lib --no-fail-fast "$@" 2>&1 | /usr/bin/grep -E "Summary|FAIL|TIMEOUT|SIGSEGV|SIGABRT|^error|Starting|panicked" | tail -60
echo "NEXTEST EXIT ${pipestatus[1]} $(date +%T)"
