#!/bin/zsh
# 🧪️ T13: native check of the raster crate after parking the half-landed adjustment-parameter leaf (chain fix, rule 30).
cd /Users/ueli/Documents/semio || exit 1
export CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-t13/target CARGO_INCREMENTAL=0
echo "QUEUED $(date '+%T')"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native t13 -- zsh -c 'echo "START $(date +%T)"; nice -n 5 cargo check -p semio-s-artifact-raster-raster --lib --tests --message-format short; echo "EXIT-NATIVE $? $(date +%T)"; nice -n 5 cargo check -p semio-s-artifact-raster-raster --features component-app-assembly --lib --tests --message-format short; echo "EXIT-NATIVE-APP $? $(date +%T)"'
echo ALL_DONE
