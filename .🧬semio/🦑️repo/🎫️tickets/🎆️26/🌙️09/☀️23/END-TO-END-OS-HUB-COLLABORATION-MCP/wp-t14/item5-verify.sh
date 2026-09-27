#!/bin/zsh
# ✅️ T14 item 5: re-measure the window-2 landings on the current tree through the fleet `native` lane (build-fleet-b):
# LC F1 helper (`drain_maintenance_pressure` + writer/jack/vcs typing laws), T13 F4 viewer refusal (plugin + dag/raster
# viewer laws), T13 wfc solve-law clock (job clock law, wfc ×5 inferences, bitmap relay law, fill laws with the app feature).
cd /Users/ueli/Documents/semio || exit 2
export CARGO_INCREMENTAL=0 RUST_MIN_STACK=33554432
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b"
export CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-t14/target
export OUT="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-t14-logs"
export WFC_LIST="-p semio-s-artifact-wfc-2d -p semio-s-artifact-wfc-grid2d -p semio-s-artifact-wfc-bitmap -p semio-s-artifact-wfc-3d -p semio-s-artifact-wfc-grid3d"
echo "START item5 $(date '+%H:%M:%S')"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native t14 -- zsh -c '
  echo "LANE $(date +%T)"
  nice -n 15 cargo check --keep-going -p semio-framework-job -p semio-framework-plugin -p semio-s-artifact-writer-writer -p semio-s-artifact-trinity-jack -p semio-s-artifact-vcs-vcs -p semio-s-plugin-dag -p semio-s-plugin-raster ${=WFC_LIST} --features semio-framework-plugin/artifact-app-testing,semio-s-artifact-trinity-jack/component-app-assembly --lib --tests --message-format short > $OUT/item5-check-1.txt 2>&1; echo "EXIT-CHECK $? $(date +%T)"
  nice -n 15 cargo check --keep-going ${=WFC_LIST} --features component-app-assembly --lib --tests --message-format short > $OUT/item5-check-wfc-app-1.txt 2>&1; echo "EXIT-CHECK-WFC-APP $? $(date +%T)"
  nice -n 15 cargo test --no-fail-fast -p semio-s-artifact-writer-writer -p semio-s-artifact-trinity-jack -p semio-s-artifact-vcs-vcs --features semio-s-artifact-trinity-jack/component-app-assembly --lib -- a_typing_run_longer_than_the_edit_ledger > $OUT/item5-f1-laws-1.txt 2>&1; echo "EXIT-F1-LAWS $? $(date +%T)"
  nice -n 15 cargo test --no-fail-fast -p semio-s-plugin-dag -p semio-s-plugin-raster --lib -- viewer_never_mutates > $OUT/item5-f4-laws-1.txt 2>&1; echo "EXIT-F4-LAWS $? $(date +%T)"
  nice -n 15 cargo test --no-fail-fast -p semio-framework-job --lib -- the_logical_clock > $OUT/item5-clock-law-1.txt 2>&1; echo "EXIT-CLOCK-LAW $? $(date +%T)"
  nice -n 15 cargo test --no-fail-fast ${=WFC_LIST} --lib -- inferences > $OUT/item5-wfc-inferences-1.txt 2>&1; echo "EXIT-WFC-INFERENCES $? $(date +%T)"
  nice -n 15 cargo test --no-fail-fast ${=WFC_LIST} --features component-app-assembly --lib -- fill > $OUT/item5-wfc-fill-app-1.txt 2>&1; echo "EXIT-WFC-FILL-APP $? $(date +%T)"'
echo "END item5 $(date '+%H:%M:%S')"
