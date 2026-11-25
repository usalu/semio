#!/usr/bin/env bash
# 🧪️ Runs the ledger unit laws (tool-machine lib and its companion) and the plugin follow laws natively (separate target dirs).
T="$(cd "$(dirname "$0")" && pwd)"; mkdir -p /c/t/tmp "$T/🗑️generated"
export TMP='C:\t\tmp' TEMP='C:\t\tmp' TMPDIR='C:\t\tmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
C=(--config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""')
cd "/c/semio/🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust" || exit 2
CARGO_TARGET_DIR='C:\t\chk' cargo test -j 2 "${C[@]}" -p semio-framework-tool-machine --lib > "$T/🗑️generated/test-ledgers.log" 2>&1; echo "rc=$?" >> "$T/🗑️generated/test-ledgers.log"
cd "/c/semio/🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🥣️companion/📦️packages/🦀️rust" || exit 2
CARGO_TARGET_DIR='C:\t\chk' cargo test -j 2 "${C[@]}" -p semio-framework-tool-machine-tests > "$T/🗑️generated/test-companion.log" 2>&1; echo "rc=$?" >> "$T/🗑️generated/test-companion.log"
cd "/c/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🍎️tests-01/📦️packages/🦀️rust" || exit 2
CARGO_TARGET_DIR='C:\t\law' cargo test -j 2 "${C[@]}" -p semio-plugin-tests-01 --lib follow > "$T/🗑️generated/test-follow.log" 2>&1; echo "rc=$?" >> "$T/🗑️generated/test-follow.log"
