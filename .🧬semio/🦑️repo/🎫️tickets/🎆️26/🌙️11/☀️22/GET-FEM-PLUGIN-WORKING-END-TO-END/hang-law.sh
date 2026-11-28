#!/usr/bin/env bash
# 🧪️ Runs one native cargo test target privately. Usage: hang-law.sh <3d|2d|fw> <package> <test-target|--lib> <filter> <tag>
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; mkdir -p "$T/🗑️generated/hang" /c/t/hangtmp
export CARGO_BUILD_BUILD_DIR='C:\t\hang' TMP='C:\t\hangtmp' TEMP='C:\t\hangtmp' TMPDIR='C:\t\hangtmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
A="/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts"
case "$1" in 2d) DIR="$A/◻️2d";; 3d) DIR="$A/🧊️3d";; fw) DIR="/c/semio/🧰️framework";; esac
LOG="$T/🗑️generated/hang/law-$5.log"
if [ "$3" = "--lib" ]; then SEL="--lib"; else SEL="--test $3"; fi
(cd "$DIR" && CARGO_TARGET_DIR='C:\t\hang' cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p "$2" $SEL -- --nocapture "$4") > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
