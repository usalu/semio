#!/usr/bin/env bash
# Runs the window config capture laws natively (roster partitions for unaddressed windows).
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
cd "/c/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧿️original/📦️packages/🦀️rust" || exit 2
mkdir -p /c/t/tmp "$T/🗑️generated"
export TMP='C:\t\tmp' TEMP='C:\t\tmp' TMPDIR='C:\t\tmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_TARGET_DIR='C:\t\wcfg' CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER=
cargo --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' test -j 3 -p semio-plugin-window-config-original --lib -- capture > "$T/🗑️generated/window-roster-test.log" 2>&1
echo "rc=$?" >> "$T/🗑️generated/window-roster-test.log"
