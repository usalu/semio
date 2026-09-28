#!/bin/zsh
# ✅️ T14 session-14b item 7 (one native-lane hold, hard 28-min deadline, resumable like overlay-hold.sh): re-measure the
# window-2 landings on the LIVE tree through build-fleet-b + the private target — LC F1 (writer/jack/vcs typing-run laws),
# T13 F4 (dag/raster viewer refusal), T13 wfc solve clock (job clock law, wfc ×5 inferences, fill laws with the app feature).
# First step: the rule-22 test-only fix gate (`semio-framework-plugin --lib --tests`, `plugin-tests/plugin-lib-tests.py`).
# State: `$L/s14b-native-state.txt`. usage: native-hold.sh <run-tag> [step…]
L="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-t14-logs"
T=/Users/ueli/Documents/semio/.tmp-ticket/wp-t14
TAG="$1"; shift
STEPS=(PLUGIN-TESTS F1 F4 CLOCK WFC-INF WFC-FILL); (( $# )) && STEPS=("$@")
export CARGO_INCREMENTAL=0 RUST_MIN_STACK=33554432 NX_DAEMON=false
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=$T/target
DEADLINE=$(( $(date +%s) + 28 * 60 ))
STATE=$L/s14b-native-state.txt; touch $STATE
WFC=(-p semio-s-artifact-wfc-2d -p semio-s-artifact-wfc-grid2d -p semio-s-artifact-wfc-bitmap -p semio-s-artifact-wfc-3d -p semio-s-artifact-wfc-grid3d)
run() { python3 $T/deadline.py $DEADLINE nice -n 15 "$@"; }
step() {
  case "$1" in
    PLUGIN-TESTS) run cargo check -p semio-framework-plugin --lib --tests --message-format short ;;
    F1) run cargo test --no-fail-fast -p semio-s-artifact-writer-writer -p semio-s-artifact-trinity-jack -p semio-s-artifact-vcs-vcs --features semio-s-artifact-trinity-jack/component-app-assembly --lib -- a_typing_run_longer_than_the_edit_ledger ;;
    F4) run cargo test --no-fail-fast -p semio-s-plugin-dag -p semio-s-plugin-raster --lib -- viewer_never_mutates ;;
    CLOCK) run cargo test --no-fail-fast -p semio-framework-job --lib -- the_logical_clock ;;
    WFC-INF) run cargo test --no-fail-fast ${WFC[@]} --lib -- inferences ;;
    WFC-FILL) run cargo test --no-fail-fast ${WFC[@]} --features component-app-assembly --lib -- fill ;;
    *) echo "unknown step $1"; return 2 ;;
  esac
}
cd /Users/ueli/Documents/semio || exit 2
echo "HOLD $TAG $(date +%T)"
for s in $STEPS; do
  if /usr/bin/grep -q "^$s " $STATE; then continue; fi
  if [ $(date +%s) -ge $DEADLINE ]; then echo "DEADLINE $(date +%T) next=$s"; break; fi
  step $s > $L/s14b-$TAG-$s.txt 2>&1; rc=$?
  echo "$s $rc $(date +%T)"
  if [ $rc -eq 124 ]; then echo "DEADLINE-KILLED $s $(date +%T)"; break; fi
  echo "$s $rc $(date +%T) $TAG" >> $STATE
done
echo "END $TAG $(date +%T)"
