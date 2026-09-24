#!/bin/zsh
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h9/target RUST_MIN_STACK=268435456
echo "=== start $(date +%T) filter: $*"
cargo test -p semio-hub --lib --bin os-hub --features native-artifact-execution,integration-fixtures --no-fail-fast -- "$@" 2>&1 | /usr/bin/grep -v "^warning\|^ *|\|^ *= \|^ *-->\|^$\|^help\|^[0-9]* [-+|]"
echo "EXIT ${pipestatus[1]}"
echo "=== done $(date +%T)"
