#!/bin/zsh
# 🧪️ T13: parity (oracle + subject + comparison) of one or more cases, rule-26 build-dir, exhaustive level.
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test" || exit 1
export CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b CARGO_INCREMENTAL=0 SEMIO_TEST_LEVEL=exhaustive PYTHONDONTWRITEBYTECODE=1
for c in "$@"; do
  echo "START $c $(date '+%T')"
  nice -n 15 bun ./📜️script.ts parity --case "$c"
  echo "EXIT $c $? $(date '+%T')"
done
echo ALL_DONE
