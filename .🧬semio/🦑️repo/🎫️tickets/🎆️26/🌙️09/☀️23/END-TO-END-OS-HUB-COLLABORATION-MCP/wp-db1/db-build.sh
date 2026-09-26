#!/bin/zsh
# DB1: build the db lib test binary (all drivers) on the DB1 private target; prints the test executable.
cd /Users/ueli/Documents/semio
export CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-db1/target RUST_MIN_STACK=268435456
echo "=== build start $(date +%T)"
nice -n 10 cargo test -p semio-framework-os-kernel-db --features sqlite,postgres,neo4j --lib --no-run "$@" 2>&1 | /usr/bin/grep -E "^error|^warning: unused|Executable|Finished|error\[" | head -60
echo "BUILD EXIT ${pipestatus[1]} $(date +%T)"
