#!/usr/bin/env bash
# Native cargo test of any package/target. Usage: fc-pkg.sh <3d|2d> <dirtag> <logtag> <cargo test args after -j 3 ...> ; filters after "--"
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; mkdir -p "$M/🗑️generated/fc"
D="$2"; mkdir -p "/c/t/${D}tmp"
W='C:\t\'
export CARGO_UNSTABLE_BUILD_DIR_NEW_LAYOUT=false CARGO_UNSTABLE_FINE_GRAIN_LOCKING=false CARGO_BUILD_BUILD_DIR="$W$D" TMP="$W${D}tmp" TEMP="$W${D}tmp" TMPDIR="$W${D}tmp" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728 RUST_BACKTRACE=1
A="/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts"
if [ "$1" = 2d ]; then DIR="$A/◻️2d"; else DIR="$A/🧊️3d"; fi
LOG="$M/🗑️generated/fc/$1-$3.log"
shift 3
for try in 1 2 3; do
(cd "$DIR" && CARGO_TARGET_DIR="$W$D" cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' "$@") > "$LOG" 2>&1
rc=$?
if [ $rc -ne 0 ] && grep -q "STATUS_ACCESS_VIOLATION" "$LOG"; then continue; fi
break
done
echo "rc=$rc" >> "$LOG"
