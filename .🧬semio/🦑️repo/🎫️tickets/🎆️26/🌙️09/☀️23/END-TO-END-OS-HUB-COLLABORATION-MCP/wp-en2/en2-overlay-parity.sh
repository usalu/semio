#!/bin/zsh
# ⚖️ EN2 overlay parity of the prepared sets: every case whose verdict the patches change, run by the overlay's own test
# platform (oracle + subject + comparison) with private build/target dirs. Case names are the case directory names.
# usage: zsh en2-overlay-parity.sh <case>…   (inside an overlay-lane hold)
set -u
OVERLAY="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-en2-overlay"
export CARGO_BUILD_BUILD_DIR="$OVERLAY/.en2-cargo/build"
export CARGO_TARGET_DIR="$OVERLAY/.en2-cargo/target"
export CARGO_INCREMENTAL=0
export SEMIO_TEST_LEVEL=exhaustive
export PYTHONDONTWRITEBYTECODE=1
export SEMIO_PYTHON="/Users/ueli/Documents/semio/.venv/bin/python3"
export SEMIO_ORACLE_OPENSTUDIO_ROOT="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/oracles/openstudio-3.11.0-darwin-arm64"
cd "$OVERLAY/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test" || exit 90
failed=0
for c in "$@"; do
  echo "[en2] START $c $(date '+%H:%M:%S')"
  nice -n 15 bun ./📜️script.ts parity --case "$c"
  rc=$?
  echo "[en2] EXIT $c $rc $(date '+%H:%M:%S')"
  [ $rc -ne 0 ] && failed=$(( failed + 1 ))
done
echo "[en2] ALL_DONE failed=$failed"
exit $failed
