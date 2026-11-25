#!/usr/bin/env bash
# ⏸️ Independently verifies the deferred host aborts: the tool-machine ledger laws plus the plugin crates that consume them, natively (separate target dir).
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"; mkdir -p /c/t/tmp "$G"
export TMP='C:\t\tmp' TEMP='C:\t\tmp' TMPDIR='C:\t\tmp' CARGO_INCREMENTAL=0 CARGO_TARGET_DIR='C:\t\abv' RUSTC_WRAPPER= RUST_MIN_STACK=134217728
cd "/c/semio/🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust" || exit 2
cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-framework-tool-machine --lib > "$G/verify-ledger.log" 2>&1; echo "rc=$?" >> "$G/verify-ledger.log"
cd "/c/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🖼️render/📦️packages/🦀️rust" || exit 2
cargo check -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-plugin-instance-render -p semio-plugin-instance-typed-ops -p semio-plugin-typing-edit > "$G/verify-plugin.log" 2>&1; echo "rc=$?" >> "$G/verify-plugin.log"
