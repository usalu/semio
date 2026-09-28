#!/bin/zsh
# 🔬️ U7 probe (native lane, ONE hold): lands the two test-only laws (`u7-laws.py`), compiles them against the live tree,
# keeps each law whose test target compiles (landing rows), reverts one that does not, and runs both to name the stale
# seated examples (stdio sweep + draw demo). Capture: wp-u7/generated/<name>.txt.
cd /Users/ueli/Documents/semio || exit 2
capture=".tmp-ticket/wp-u7/generated/$1.txt"
export NX_DAEMON=false CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.tmp-ticket/wp-u7/target"
{ echo "QUEUED $(date '+%H:%M:%S')"; zsh .tmp-ticket/📜️fleet-mutex.sh native u7 -- zsh -c '
while [ "$(ps -axo command | /usr/bin/grep -c "^[^ ]*rustc ")" -gt 14 ]; do sleep 30; done
echo "START $(date "+%H:%M:%S") rustc=$(ps -axo command | /usr/bin/grep -c "^[^ ]*rustc ")"
laws=.tmp-ticket/wp-u7/u7-laws.py
python3 $laws --write
nice -n 15 cargo test --no-fail-fast -p semio-s-plugin-stdio --test example_sweep --no-run; rc=$?; echo "STDIO-BUILD rc=$rc $(date "+%H:%M:%S")"
if [ $rc -ne 0 ]; then python3 $laws --revert --part stdio-sweep; echo "STDIO-SWEEP REVERTED"; else nice -n 15 cargo test --no-fail-fast -p semio-s-plugin-stdio --test example_sweep; echo "STDIO-SWEEP rc=$? $(date "+%H:%M:%S")"; fi
nice -n 15 cargo test --no-fail-fast -p semio-s-artifact-draw-drawing --lib --no-run; rc=$?; echo "DRAW-BUILD rc=$rc $(date "+%H:%M:%S")"
if [ $rc -ne 0 ]; then python3 $laws --revert --part draw-demo; echo "DRAW-DEMO REVERTED"; else nice -n 15 cargo test --no-fail-fast -p semio-s-artifact-draw-drawing --lib -- demo_decodes_into_a_non_empty_drawing dsl_round_trips_semio_example_fixture; echo "DRAW-LAW rc=$? $(date "+%H:%M:%S")"; fi
python3 $laws --dry-run
echo "END $(date "+%H:%M:%S")"
'; } > "$capture" 2>&1
