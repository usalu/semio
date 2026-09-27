#!/bin/zsh
# 🦀️ Z4: one cargo command in the fleet's native lane (preamble 14 rule 3), build-fleet-b, no incremental, private target
# dir, niced, with start/exit stamps. usage: zsh z4-cargo.sh <cargo args…>
cd /Users/ueli/Documents/semio || exit 1
export CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b
export CARGO_INCREMENTAL=0
export CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-z4/target
exec zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native z4 -- zsh -c 'echo "=== start $(date +%T) cargo $*"; nice -n 15 cargo "$@"; rc=$?; echo "=== EXIT $rc $(date +%T)"; exit $rc' z4 "$@"
