#!/usr/bin/env bash
set -u
export PATH="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims:$PATH"
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🏙️bin-01/📦️packages/🦀️rust" || exit 1
out="${OUT:-/dev/stdout}"
cargo test -j 2 --no-run >"$out" 2>&1 || exit 2
cargo test -j 2 "$@" -- --test-threads=2 >>"$out" 2>&1 &
pid=$!
( sleep "${WATCHDOG_SECONDS:-600}"; echo "WATCHDOG: killing after ${WATCHDOG_SECONDS:-600}s" >>"$out"; pkill -P "$pid" 2>/dev/null; pkill -f "semio_os_store_tests_01" 2>/dev/null ) &
dog=$!
wait "$pid"; code=$?
kill "$dog" 2>/dev/null
exit "$code"
