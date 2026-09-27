#!/bin/zsh
# ⚡️ EN2 overlay proof of the energy epJSON + test-raw-routing patches: check/test the test host, regenerate the committed epJSON fixtures with the patched codec,
# then run every exporter/importer epJSON unit law. Private build-dir + target-dir inside the overlay (preamble rule 3).
# usage: zsh en2-overlay-energy.sh   (run through `📜️fleet-mutex.sh overlay en2 -- …`)
set -u
OVERLAY="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-en2-overlay"
export CARGO_BUILD_BUILD_DIR="$OVERLAY/.en2-cargo/build"
export CARGO_TARGET_DIR="$OVERLAY/.en2-cargo/target"
export CARGO_INCREMENTAL=0
cd "$OVERLAY" || exit 90
echo "[en2] $(date '+%H:%M:%S') test host (per-scenario raw routing)"
nice -n 15 cargo test --manifest-path "$OVERLAY/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust/Cargo.toml" --no-fail-fast
host=$?
echo "[en2] $(date '+%H:%M:%S') test host rc=$host"
echo "[en2] $(date '+%H:%M:%S') regenerate"
SEMIO_ENERGY_EPJSON_REGENERATE=1 nice -n 15 cargo test -p semio-s-artifact-energy-model --lib --no-fail-fast -- regenerate_committed_epjson_fixtures
regen=$?
echo "[en2] $(date '+%H:%M:%S') regenerate rc=$regen"
nice -n 15 cargo test -p semio-s-artifact-energy-model --lib --no-fail-fast -- epjson
laws=$?
echo "[en2] $(date '+%H:%M:%S') epjson laws rc=$laws"
zsh /Users/ueli/Documents/semio/.tmp-ticket/wp-en2/en2-overlay-parity.sh 🏛️export-epjson-runs-in-energyplus 🗜️breach-cache-envelope
parity=$?
echo "[en2] $(date '+%H:%M:%S') parity rc=$parity"
exit $(( host + regen + laws + parity ))
