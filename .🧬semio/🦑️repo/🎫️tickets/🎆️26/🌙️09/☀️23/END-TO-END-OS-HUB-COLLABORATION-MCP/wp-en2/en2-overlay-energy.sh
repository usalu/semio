#!/bin/zsh
# ⚡️ EN2 overlay proof of the energy epJSON + test-raw-routing patches, one phase per overlay-lane hold, every phase capped
# at 28 min by `en2-deadline.py` (process-group kill). Private build-dir + target-dir inside the overlay (preamble rule 3).
#   build   — test host check + tests (per-scenario raw routing), energy-model `--lib --tests` build
#   laws    — regenerate the committed epJSON fixtures with the patched codec, every exporter/importer epJSON law, then parity
#   parity  — parity of the epJSON EnergyPlus case and the statutes raw-routing case
# usage: zsh 📜️fleet-mutex.sh overlay en2 -- zsh en2-overlay-energy.sh <phase>
set -u
OVERLAY="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-en2-overlay"
HERE="/Users/ueli/Documents/semio/.tmp-ticket/wp-en2"
export CARGO_BUILD_BUILD_DIR="$OVERLAY/.en2-cargo/build"
export CARGO_TARGET_DIR="$OVERLAY/.en2-cargo/target"
export CARGO_INCREMENTAL=0
cd "$OVERLAY" || exit 90
rc=0
step() { echo "[en2] $(date '+%H:%M:%S') START $*"; python3 "$HERE/en2-deadline.py" 1680 nice -n 15 "$@"; local r=$?; echo "[en2] $(date '+%H:%M:%S') rc=$r"; rc=$(( rc + r )); }
case "${1:-}" in
  build)
    step cargo test --manifest-path "$OVERLAY/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust/Cargo.toml" --no-fail-fast
    step cargo test -p semio-s-artifact-energy-model --lib --no-run ;;
  laws)
    export SEMIO_ENERGY_EPJSON_REGENERATE=1
    step cargo test -p semio-s-artifact-energy-model --lib --no-fail-fast -- regenerate_committed_epjson_fixtures
    unset SEMIO_ENERGY_EPJSON_REGENERATE
    step cargo test -p semio-s-artifact-energy-model --lib --no-fail-fast -- epjson
    step zsh "$HERE/en2-overlay-parity.sh" 🏛️export-epjson-runs-in-energyplus 🗜️breach-cache-envelope ;;
  parity)
    step zsh "$HERE/en2-overlay-parity.sh" 🏛️export-epjson-runs-in-energyplus 🗜️breach-cache-envelope ;;
  *) echo "usage: en2-overlay-energy.sh build|laws|parity"; exit 2 ;;
esac
exit $rc
