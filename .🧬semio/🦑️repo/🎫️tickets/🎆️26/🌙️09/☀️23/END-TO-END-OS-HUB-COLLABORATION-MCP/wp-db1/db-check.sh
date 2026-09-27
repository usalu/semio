#!/bin/zsh
# DB1: cargo check of the db crate (all drivers, lib + tests) and semio-framework-async on build-fleet-b (rule 26), nice 10.
cd /Users/ueli/Documents/semio
export CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-db1/target
echo "=== check start $(date +%T)"
nice -n 10 cargo check -p semio-framework-os-kernel-db -p semio-framework-async --features semio-framework-os-kernel-db/sqlite,semio-framework-os-kernel-db/postgres,semio-framework-os-kernel-db/neo4j --lib --tests --message-format short 2>&1 | /usr/bin/grep -E "^error|: error|warning: unused (variable|import)|never used|Finished|could not compile" | head -60
echo "CHECK EXIT ${pipestatus[1]} $(date +%T)"
