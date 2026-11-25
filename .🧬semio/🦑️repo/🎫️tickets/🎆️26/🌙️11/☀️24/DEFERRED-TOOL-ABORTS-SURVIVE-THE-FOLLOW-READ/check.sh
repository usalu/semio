#!/usr/bin/env bash
# 🧾️ Type-checks the tool-machine ledgers (with tests) and the instance render that settles deferred tool aborts natively (separate target dir).
T="$(cd "$(dirname "$0")" && pwd)"; mkdir -p /c/t/tmp "$T/🗑️generated"
cd "/c/semio/🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust" || exit 2
export TMP='C:\t\tmp' TEMP='C:\t\tmp' TMPDIR='C:\t\tmp' CARGO_INCREMENTAL=0 CARGO_TARGET_DIR='C:\t\chk' RUSTC_WRAPPER= RUST_MIN_STACK=134217728
cargo check -j 2 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-framework-tool-machine --tests ${FEATURES:+--features "$FEATURES"} > "$T/🗑️generated/check-ledgers.log" 2>&1
echo "rc=$?" >> "$T/🗑️generated/check-ledgers.log"
cd "/c/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🖼️render/📦️packages/🦀️rust" || exit 2
cargo check -j 2 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-plugin-instance-render -p semio-plugin-typing-edit -p semio-plugin-instance-typed-ops > "$T/🗑️generated/check-render.log" 2>&1
echo "rc=$?" >> "$T/🗑️generated/check-render.log"
