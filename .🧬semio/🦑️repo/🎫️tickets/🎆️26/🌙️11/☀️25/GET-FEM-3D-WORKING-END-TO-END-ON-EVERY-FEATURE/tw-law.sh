#!/usr/bin/env bash
# 🧪️ FEM 3D/2D twin fix laws. Usage: tw-law.sh <3d|2d> <tag> <filters...>
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; mkdir -p "$M/🗑️generated/tw" /c/t/twtmp
export CARGO_BUILD_BUILD_DIR='C:\t\tw' TMP='C:\t\twtmp' TEMP='C:\t\twtmp' TMPDIR='C:\t\twtmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
A="/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts"
if [ "$1" = 2d ]; then DIR="$A/◻️2d"; PKG=semio-s-artifact-fem-2d-editor; else DIR="$A/🧊️3d"; PKG=semio-s-artifact-fem-3d-editor; fi
TGT="${TGT:-editor_app}"; LOG="$M/🗑️generated/tw/$1-$TGT.log"; shift 2
for try in 1 2 3; do
(cd "$DIR" && CARGO_TARGET_DIR='C:\t\tw' cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p "$PKG" --test "$TGT" -- "$@") > "$LOG" 2>&1
rc=$?
if [ $rc -ne 0 ] && grep -q "STATUS_ACCESS_VIOLATION" "$LOG"; then continue; fi
break
done
echo "rc=$rc" >> "$LOG"
