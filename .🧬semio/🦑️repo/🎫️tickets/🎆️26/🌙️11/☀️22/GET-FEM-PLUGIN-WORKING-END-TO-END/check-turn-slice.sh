#!/usr/bin/env bash
# ⏳️ Type-checks the reactor turn crate (typed-operation turn slice) natively in a separate target dir.
T="$(cd "$(dirname "$0")" && pwd)"; mkdir -p /c/t/tmp "$T/🗑️generated"
cd "/c/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/📦️packages/🦀️rust" || exit 2
export TMP='C:\t\tmp' TEMP='C:\t\tmp' TMPDIR='C:\t\tmp' CARGO_INCREMENTAL=0 CARGO_TARGET_DIR='C:\t\chk' RUSTC_WRAPPER= RUST_MIN_STACK=134217728
cargo check -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-plugin-reactor-turn > "$T/🗑️generated/check-turn-slice.log" 2>&1
echo "rc=$?" >> "$T/🗑️generated/check-turn-slice.log"
