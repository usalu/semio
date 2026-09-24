#!/bin/zsh
# Hub suite on the H9 private target: `hub-suite.sh <label> <cargo feature args...> [-- <libtest args>]`.
cd /Users/ueli/Documents/semio
LABEL=$1; shift
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h9/target RUST_MIN_STACK=268435456
export SEMIO_TEST_ARTIFACT_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h9/generated/test-artifacts-$LABEL
mkdir -p "$SEMIO_TEST_ARTIFACT_DIR" && chmod 700 "$SEMIO_TEST_ARTIFACT_DIR"
echo "=== start $(date +%T) label=$LABEL args: $*"
cargo test -p semio-hub --no-fail-fast "$@" 2>&1 | /usr/bin/grep -v "^warning\|^ *|\|^ *= \|^ *-->\|^$\|^help\|^[0-9]* [-+|]"
echo "EXIT ${pipestatus[1]}"
echo "=== done $(date +%T)"
