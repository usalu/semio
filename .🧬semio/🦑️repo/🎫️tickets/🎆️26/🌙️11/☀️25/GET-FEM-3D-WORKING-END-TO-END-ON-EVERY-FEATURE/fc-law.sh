#!/usr/bin/env bash
# Native law runner for FEM actions/clipboard/resolve. Usage: fc-law.sh <3d|2d> <test target> <filter> <tag>
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; mkdir -p "$M/🗑️generated/fc" /c/t/fctmp
export CARGO_UNSTABLE_BUILD_DIR_NEW_LAYOUT=false CARGO_UNSTABLE_FINE_GRAIN_LOCKING=false CARGO_BUILD_BUILD_DIR='C:\t\fc' TMP='C:\t\fctmp' TEMP='C:\t\fctmp' TMPDIR='C:\t\fctmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
A="/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts"
if [ "$1" = 2d ]; then DIR="$A/◻️2d"; PKG=semio-s-artifact-fem-2d-editor; else DIR="$A/🧊️3d"; PKG=semio-s-artifact-fem-3d-editor; fi
LOG="$M/🗑️generated/fc/$1-$4.log"
for try in 1 2 3; do
(cd "$DIR" && CARGO_TARGET_DIR='C:\t\fc' cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p "$PKG" --test "$2" -- --nocapture ${3//,/ }) > "$LOG" 2>&1
rc=$?
if [ $rc -ne 0 ] && grep -q "STATUS_ACCESS_VIOLATION" "$LOG"; then continue; fi
break
done
echo "rc=$rc" >> "$LOG"
