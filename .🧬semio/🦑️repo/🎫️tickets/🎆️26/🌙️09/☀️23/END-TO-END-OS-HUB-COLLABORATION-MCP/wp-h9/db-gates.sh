#!/bin/zsh
# db gates: plain in-process lib tests, then nextest, on the H9 private target.
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h9/target RUST_MIN_STACK=268435456
echo "=== lib start $(date +%T)"
cargo test -p semio-framework-os-kernel-db --lib --no-fail-fast "$@" 2>&1 | /usr/bin/grep -v "^warning\|^ *|\|^ *= \|^ *-->\|^$\|^help\|^[0-9]* [-+|]" | /usr/bin/grep -E "test result|FAILED|panicked|^error|failures:|^    [a-z_:]+$|Running"
echo "LIB EXIT ${pipestatus[1]} $(date +%T)"
echo "=== nextest start $(date +%T)"
cargo nextest run -p semio-framework-os-kernel-db --lib --no-fail-fast "$@" 2>&1 | /usr/bin/grep -E "Summary|FAIL|TIMEOUT|SIGSEGV|SIGABRT|^error|Starting" | tail -40
echo "NEXTEST EXIT ${pipestatus[1]} $(date +%T)"
echo "=== done $(date +%T)"
