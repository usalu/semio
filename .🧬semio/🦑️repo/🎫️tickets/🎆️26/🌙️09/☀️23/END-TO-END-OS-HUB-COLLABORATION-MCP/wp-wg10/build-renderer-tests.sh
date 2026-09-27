#!/bin/zsh
# WG10 s13: builds the renderer lib test binary (native live laws: gate, journey, cross-shell) and keeps a durable copy of it.
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-wg10/target CARGO_BUILD_BUILD_DIR=${WG10_BUILD_DIR:-/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b}
echo "START $(date '+%F %T')"
nice -n 10 cargo test -p semio-framework-os-renderer-wgpu --lib --no-run --message-format short 2>&1 | tee /dev/stderr | /usr/bin/grep -o 'Executable unittests[^(]*([^)]*)' | tail -1 | sed 's/.*(\(.*\))/\1/' > /tmp/wg10-renderer-test-path.txt
BIN=$(cat /tmp/wg10-renderer-test-path.txt)
case "$BIN" in /*) ;; *) BIN="/Users/ueli/Documents/semio/$BIN" ;; esac
[ -x "$BIN" ] || { echo "RC=1 no test binary ($BIN) $(date '+%F %T')"; exit 1; }
mkdir -p "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-wg10-bin"
cp "$BIN" "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-wg10-bin/renderer-tests"
echo "BIN $BIN"
echo "RC=0 END $(date '+%F %T')"
