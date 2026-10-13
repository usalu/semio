#!/bin/bash
# 🦀️ Artifact-tests cargo wrapper: private target and build dirs under ⚡️cache/at, one fleet slot, a deadline (exit 124 resumes on rerun).
# usage: 🦀️cargo.sh <seconds> <cargo args...>
cd /Users/ueli/Documents/semio || exit 2
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"
LIMIT="$1"; shift
export CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/⚡️cache/at/t" CARGO_BUILD_BUILD_DIR="$PWD/.🧬semio/🦑️repo/⚡️cache/at/b" CARGO_TERM_COLOR=never
exec bash "$T/🚦️cargo-slot.sh" at-c -- bash "$T/wave0-feature-drift/⏱️bounded.sh" "$LIMIT" -- cargo "$@"
