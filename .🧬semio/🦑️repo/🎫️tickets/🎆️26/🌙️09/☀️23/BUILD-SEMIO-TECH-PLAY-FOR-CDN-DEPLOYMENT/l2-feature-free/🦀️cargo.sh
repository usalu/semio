#!/bin/bash
# 🦀️ Wave L2 cargo wrapper: private target and build dirs under ⚡️cache/l2, one fleet slot, a deadline (exit 124 on expiry).
# usage: 🦀️cargo.sh <seconds> <cargo args...>
cd /Users/ueli/Documents/semio || exit 2
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"
LIMIT="$1"; shift
export CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/⚡️cache/l2/target" CARGO_BUILD_BUILD_DIR="$PWD/.🧬semio/🦑️repo/⚡️cache/l2/build" CARGO_TERM_COLOR=never
exec bash "$T/🚦️cargo-slot.sh" l2 -- bash "$T/wave0-feature-drift/⏱️bounded.sh" "$LIMIT" -- cargo "$@"
