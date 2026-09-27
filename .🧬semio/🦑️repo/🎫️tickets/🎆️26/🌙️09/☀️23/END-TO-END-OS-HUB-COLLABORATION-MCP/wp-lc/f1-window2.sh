#!/bin/zsh
# ⌨️ LC — F1 law harness landing (window 2): ONE native-lane hold = native check of every touched crate, then the three laws.
cd /Users/ueli/Documents/semio || exit 2
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-lc/target RUST_MIN_STACK=33554432
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b"
out=".🧬semio/🌐hub/s13-lc-laws"
echo "START f1-window2 $(date '+%H:%M:%S')"
zsh .tmp-ticket/📜️fleet-mutex.sh native lc -- zsh -c "nice -n 15 cargo check -p semio-framework-plugin -p semio-s-artifact-writer-writer -p semio-s-artifact-trinity-jack -p semio-s-artifact-vcs-vcs --features semio-framework-plugin/artifact-app-testing,semio-s-artifact-trinity-jack/component-app-assembly --lib --tests --message-format short > $out/f1-check-2.txt 2>&1; echo CHECK rc=\$?; nice -n 15 cargo test -p semio-s-artifact-writer-writer -p semio-s-artifact-trinity-jack -p semio-s-artifact-vcs-vcs --features semio-s-artifact-trinity-jack/component-app-assembly --lib --no-fail-fast -- a_typing_run_longer_than_the_edit_ledger > $out/f1-laws-2.txt 2>&1; echo LAWS rc=\$?"
echo "END f1-window2 $(date '+%H:%M:%S')"
