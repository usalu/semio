#!/usr/bin/env bash
# Runs one law natively in private ls dirs. Usage: ls-law.sh <filter> <tag> <package> [--lib|<test target>]; LSDIR selects the cargo workspace dir.
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; mkdir -p "$M/🗑️generated/ls" /c/t/lstmp
export CARGO_BUILD_BUILD_DIR='C:\t\ls' TMP='C:\t\lstmp' TEMP='C:\t\lstmp' TMPDIR='C:\t\lstmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
LOG="$M/🗑️generated/ls/law-${2:-a}.log"
PKG="${3:-semio-s-artifact-fem-3d-editor-visual-job}"
if [ "${4:---lib}" = "--lib" ]; then TARGET="--lib"; else TARGET="--test $4"; fi
(cd "${LSDIR:-/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d}" && CARGO_TARGET_DIR='C:\t\ls' cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p "$PKG" $TARGET -- --nocapture "${1:-probe}") > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
