#!/bin/zsh
# 🦀️ Z3 cargo wrapper: build-fleet-b build-dir (preamble rule 26), private target, no incremental, nice 10; logs the exit code.
cd /Users/ueli/Documents/semio || exit 2
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b"
export CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.tmp-ticket/wp-z3/target"
export CARGO_INCREMENTAL=0
start=$(date +%s)
nice -n 10 cargo "$@"
rc=$?
echo "[z3-cargo] rc=$rc seconds=$(( $(date +%s) - start )) args=$*"
exit $rc
