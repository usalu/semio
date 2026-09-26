#!/bin/zsh
# Named hub lib laws on the H9 private target (niced per preamble rule 18).
# usage: lib-laws-s12.sh <filter>…
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h9/target RUST_MIN_STACK=268435456
echo "=== start $(date +%T) filter: $*"
nice -n 15 cargo test -p semio-hub --lib --features native-artifact-execution,integration-fixtures --no-fail-fast -- "$@" 2>&1 | /usr/bin/grep -v "^warning\|^ *|\|^ *= \|^ *-->\|^$\|^help\|^[0-9]* [-+|]"
echo "EXIT ${pipestatus[1]}"
echo "=== done $(date +%T)"
