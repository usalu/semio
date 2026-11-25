#!/usr/bin/env bash
# ⏱️ Runs the turn-slice and ack-wait laws natively and type-checks the crates the turn edits touch. Usage: turn-slice-laws.sh [log-suffix]
T="$(cd "$(dirname "$0")" && pwd)"; mkdir -p /c/t/tmp "$T/🗑️generated"
export TMP='C:\t\tmp' TEMP='C:\t\tmp' TMPDIR='C:\t\tmp' CARGO_INCREMENTAL=0 CARGO_TARGET_DIR='C:\t\law2' RUSTC_WRAPPER= RUST_MIN_STACK=134217728
P="/c/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
C=(--config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""')
L="$T/🗑️generated/turn-slice-laws${1:-}.log"
: > "$L"
cd "$P/⚛️reactor/🔄️turn/📦️packages/🦀️rust" || exit 2
cargo test -j 3 "${C[@]}" -p semio-plugin-reactor-turn --lib -- quiet_ starved >> "$L" 2>&1; echo "rc-turn=$?" >> "$L"
cd "$P/🧩️composition/📬️publication/📦️packages/🦀️rust" || exit 2
cargo test -j 3 "${C[@]}" -p semio-plugin-composition-publication --lib -- ack_wait >> "$L" 2>&1; echo "rc-publication=$?" >> "$L"
cd "$P/⚛️reactor/🩹️patches/📦️packages/🦀️rust" || exit 2
cargo test -j 3 "${C[@]}" -p semio-plugin-reactor-patches --lib -- reported_output_fault >> "$L" 2>&1; echo "rc-patches=$?" >> "$L"
cd "$P/🧵️instance/📦️packages/🦀️rust" || exit 2
cargo check -j 3 "${C[@]}" -p semio-plugin-instance-app -p semio-plugin-instance-jobs -p semio-plugin-runtime-ops >> "$L" 2>&1; echo "rc-check=$?" >> "$L"
echo "rc=done" >> "$L"
