#!/usr/bin/env bash
# 🧠️ Runs one framework plugin-SDK law natively in private saf dirs. Usage: sa-fw-law.sh <test filter> <log tag>
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; mkdir -p "$M/🗑️generated/sa" /c/t/safmp
export TMP='C:\t\safmp' TEMP='C:\t\safmp' TMPDIR='C:\t\safmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
LOG="$M/🗑️generated/sa/law-$2.log"
(cd "/c/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🪛️tests-09-2/📦️packages/🦀️rust" && CARGO_TARGET_DIR='C:\t\saf' cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-plugin-tests-09-2 --lib -- --nocapture "$1") > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
grep -E "^test |test result|^error(\[|:)|rc=|\[DEBUG\]" "$LOG" | head -40
