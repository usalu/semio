#!/bin/bash
# 🦀️ Wave L2 cargo wrapper: the issued launcher on the shared store (through 🚦️cargo-slot.sh) under a deadline (exit 124 on expiry).
# usage: 🦀️cargo.sh <seconds> <cargo subcommand> <args...>
cd /Users/ueli/Documents/semio || exit 2
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"
LIMIT="$1"; shift
export CARGO_TERM_COLOR=never
unset CARGO_TARGET_DIR CARGO_BUILD_BUILD_DIR
exec bash "$T/wave0-feature-drift/⏱️bounded.sh" "$LIMIT" -- bash "$T/🚦️cargo-slot.sh" l2 -- cargo "$@"
