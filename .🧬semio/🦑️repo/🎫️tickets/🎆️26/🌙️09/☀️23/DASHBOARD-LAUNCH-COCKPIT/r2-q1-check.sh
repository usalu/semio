#!/bin/sh
cd /c/git/semio
export CARGO_TARGET_DIR=/c/git/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target-fleet-q1b
export CARGO_BUILD_BUILD_DIR=$CARGO_TARGET_DIR/private-build
cargo +nightly-2026-07-07-x86_64-pc-windows-msvc check -p semio-framework-repo-dashboard --all-targets --message-format short 2>&1 | grep -E 'dashboard|^error' | grep -v '^warning: `' | sed 's|🧰️framework.🛍️products.🦑️repo.🔨️modules.🎛️dashboard.📦️packages.🦀️rust...\.\.||' | cut -c1-330 | head -${1:-40}
