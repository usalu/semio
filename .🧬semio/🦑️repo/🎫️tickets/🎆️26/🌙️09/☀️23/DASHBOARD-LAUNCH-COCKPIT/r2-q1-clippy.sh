#!/bin/sh
cd /c/git/semio
export CARGO_TARGET_DIR=/c/git/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target-fleet-q1b
export CARGO_BUILD_BUILD_DIR=$CARGO_TARGET_DIR/private-build
T="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/DASHBOARD-LAUNCH-COCKPIT"
touch "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/🦀️.rs" 2>/dev/null
cargo +nightly-2026-07-07-x86_64-pc-windows-msvc clippy -p semio-framework-repo-dashboard --all-targets --message-format short 2>&1 | grep -E 'dashboard|🎛️dashboard' | grep -E 'warning|error' | sort -u > "$T/🗑️generated/q1b-clippy-short.txt"
wc -l < "$T/🗑️generated/q1b-clippy-short.txt"
