#!/bin/zsh
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h9/target RUST_MIN_STACK=268435456
echo "=== start $(date +%T)"
cargo test -p semio-hub --lib --bin os-hub --features native-artifact-execution --no-fail-fast -- check_in credential_sign_in_reports_a_failing 2>&1 | /usr/bin/grep -v "^warning\|^ *|\|^ *= \|^ *-->\|^$"
echo "EXIT ${pipestatus[1]}"
echo "=== done $(date +%T)"
