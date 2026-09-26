#!/bin/zsh
# DB1: run ONE db lib law in place (no isolation child, no watchdog) on the DB1 private target.
# usage: db-law.sh <exact law path> [extra libtest args]
cd /Users/ueli/Documents/semio
export CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-db1/target RUST_MIN_STACK=268435456
law="$1"; shift
export SEMIO_DB_ISOLATED_LAW="$law"
echo "=== law $law start $(date +%T)"
nice -n 10 cargo test -p semio-framework-os-kernel-db --features sqlite,postgres,neo4j --lib --no-fail-fast -- --exact "$law" --nocapture --include-ignored --test-threads=1 "$@" 2>&1 | /usr/bin/grep -v "^warning\|^ *|\|^ *= \|^ *-->\|^$\|^help\|^[0-9]* [-+|]\|^\.\.\." | /usr/bin/grep -A3 -E "throughput\[|test result|FAILED|panicked|^error|^test |Running|storm|greeting"
echo "LAW EXIT ${pipestatus[1]} $(date +%T)"
