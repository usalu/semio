#!/usr/bin/env bash
# 🧪️ Runs FEM 3D example-switch law natively in private dirs C:\t\im*. Usage: im-law.sh <test filter>
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; mkdir -p "$M/🗑️generated/im" /c/t/imtmp
export CARGO_TARGET_DIR='C:\t\im' TMP='C:\t\imtmp' TEMP='C:\t\imtmp' TMPDIR='C:\t\imtmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
LOG="$M/🗑️generated/im/law.log"
(cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d" && cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-s-artifact-fem-3d-editor --lib -- "${1:-set_active_example}") > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
grep -E "^test |test result|^error(\[|:)|rc=" "$LOG" | cut -c1-240 | head -40
