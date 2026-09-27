#!/bin/zsh
# 🧪️ T13 landing window 2 gate: F4 viewer-refusal law + wfc solve-law clock (F9 stays prepared: its 1 045 id carriers need
# owner regeneration first). Native check + laws through the fleet `native` lane, then the wasm32-wasip2 fast gate.
cd /Users/ueli/Documents/semio || exit 1
export CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-t13/target CARGO_INCREMENTAL=0
export WFC_LIST="-p semio-s-artifact-wfc-2d -p semio-s-artifact-wfc-grid2d -p semio-s-artifact-wfc-bitmap -p semio-s-artifact-wfc-3d -p semio-s-artifact-wfc-grid3d"
[ "$1" = "--wasm-only" ] || zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native t13 -- zsh -c '
  nice -n 10 cargo check --keep-going -p semio-framework-job -p semio-framework-plugin ${=WFC_LIST} --lib --tests --message-format short; echo "EXIT-NATIVE $? $(date +%T)"
  nice -n 10 cargo check --keep-going ${=WFC_LIST} --features component-app-assembly --lib --tests --message-format short; echo "EXIT-NATIVE-WFC-APP $? $(date +%T)"
  nice -n 10 cargo test -p semio-framework-job --lib the_logical_clock; echo "EXIT-LAW-LOGICAL-CLOCK $? $(date +%T)"
  nice -n 10 cargo test --no-fail-fast ${=WFC_LIST} --lib inferences; echo "EXIT-LAW-WFC-INFERENCES $? $(date +%T)"'
[ "$1" = "--native-only" ] && { echo ALL_DONE; exit 0; }
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh wasm t13 -- zsh -c '
  nice -n 10 cargo check --keep-going ${=WFC_LIST} --features component-app-assembly --lib --target wasm32-wasip2 --message-format short; echo "EXIT-WASM-WFC-APP $? $(date +%T)"'
echo ALL_DONE
