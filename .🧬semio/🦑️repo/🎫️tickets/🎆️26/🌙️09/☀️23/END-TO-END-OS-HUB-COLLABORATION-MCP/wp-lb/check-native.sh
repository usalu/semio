#!/bin/zsh
# 🧪 LB native compile-atomic check (landing build-dir, rule 27): check-native.sh <capture-name> <cargo check args…>
cd /Users/ueli/Documents/semio || exit 2
capture=".tmp-ticket/wp-lb/generated/$1.txt"; shift
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-landing"
{ echo "START $(date '+%H:%M:%S') cargo check $*"; cargo check "$@" 2>&1; echo "EXIT $? $(date '+%H:%M:%S')"; } > "$capture" 2>&1
