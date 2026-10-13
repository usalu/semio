#!/bin/bash
# Runs one cargo command in the foreground through the fleet slot gate with private target/build dirs under ⚡️cache/tc.
R="/Users/ueli/Documents/semio"; C="$R/.🧬semio/🦑️repo/⚡️cache/tc"; mkdir -p "$C"
export CARGO_TARGET_DIR="$C/target" CARGO_BUILD_BUILD_DIR="$C/build" CARGO_INCREMENTAL=0
cd "$R" && exec "$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🚦️cargo-slot.sh" tc -- "$@"
