#!/usr/bin/env bash
# 🧪️ Runs the press and typing follow regression laws in the plugin companion test crate natively (separate target dir).
T="$(cd "$(dirname "$0")" && pwd)"; mkdir -p /c/t/tmp "$T/🗑️generated"
cd "/c/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🍎️tests-01/📦️packages/🦀️rust" 2>/dev/null || cd "$(dirname "$(find /c/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🍎️tests-01 -name Cargo.toml | head -1)")" || exit 2
export TMP='C:\t\tmp' TEMP='C:\t\tmp' TMPDIR='C:\t\tmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_TARGET_DIR='C:\t\law' RUSTC_WRAPPER= RUST_MIN_STACK=134217728
cargo test -j 2 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-plugin-tests-01 --lib drives_the_follow > "$T/🗑️generated/law-test.log" 2>&1
echo "rc=$?" >> "$T/🗑️generated/law-test.log"
