#!/usr/bin/env bash
# 🧠️ Runs one FEM law natively in private sa dirs. Usage: sa-law.sh <3d|2d> <package> <test filter> <log tag> [--test <target> | --lib]
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; mkdir -p "$M/🗑️generated/sa" /c/t/satmp
export CARGO_BUILD_BUILD_DIR='C:\t\sa' TMP='C:\t\satmp' TEMP='C:\t\satmp' TMPDIR='C:\t\satmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
A="/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts"
if [ "$1" = 2d ]; then DIR="$A/◻️2d"; else DIR="$A/🧊️3d"; fi
LOG="$M/🗑️generated/sa/law-$4.log"
(cd "$DIR" && CARGO_TARGET_DIR='C:\t\sa' cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p "$2" ${5:---lib} ${6} -- --nocapture "$3") > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
grep -E "^test |test result|^error(\[|:)|rc=|\[DEBUG\]" "$LOG" | head -40
