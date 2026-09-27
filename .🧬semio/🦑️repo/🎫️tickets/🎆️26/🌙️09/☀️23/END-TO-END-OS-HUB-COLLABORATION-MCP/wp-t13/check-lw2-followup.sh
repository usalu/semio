#!/bin/zsh
# 🧪️ T13 window-2 follow-up through the fleet `native` lane: the wfc `component-app-assembly` test targets after the grid2d/grid3d
# fill-test import fix, the bitmap relay timing law on the logical-clock reference solve, and the fill laws under the app feature.
cd /Users/ueli/Documents/semio || exit 1
export CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-t13/target CARGO_INCREMENTAL=0
export WFC_LIST="-p semio-s-artifact-wfc-2d -p semio-s-artifact-wfc-grid2d -p semio-s-artifact-wfc-bitmap -p semio-s-artifact-wfc-3d -p semio-s-artifact-wfc-grid3d"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native t13 -- zsh -c '
  nice -n 10 cargo check --keep-going ${=WFC_LIST} --features component-app-assembly --lib --tests --message-format short; echo "EXIT-NATIVE-WFC-APP $? $(date +%T)"
  nice -n 10 cargo test -p semio-s-artifact-wfc-bitmap --lib a_whole_solve_grant_settles_the_genesis_inference; echo "EXIT-LAW-BITMAP-RELAY $? $(date +%T)"
  nice -n 10 cargo test --no-fail-fast ${=WFC_LIST} --features component-app-assembly --lib fill; echo "EXIT-LAW-WFC-FILL-APP $? $(date +%T)"'
echo ALL_DONE
