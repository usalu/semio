#!/bin/zsh
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-p6/target RUST_MIN_STACK=33554432
args=()
for p in $(cat .tmp-ticket/wp-p6/plugin-packages.txt | grep -v -x -e "${SWEEP_EXCLUDE:-__none__}"); do args+=(-p "$p"); done
cargo nextest run "${args[@]}" --no-fail-fast --profile quick -- --skip long:: --skip exhaustive::
echo "SWEEP EXIT $?"
