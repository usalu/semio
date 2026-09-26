#!/bin/zsh
# ✅️ H10: compile-checks and law-runs the post-publish patch sets (Q1 interpreter, Q2 hardware SHA-256, guest codec-app
# resolution) applied to a COPY of the repository, with a private target + build dir, so the tree is never touched.
# usage: patch-check.sh <repository copy> <capture>
C=$1; OUT=$2
cd "$C"
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h10/target/patch-check CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h10/target/patch-check-build
echo "=== start $(date +%T)" > "$OUT"
nice -n 15 cargo test -p semio-framework-hash --no-fail-fast >> "$OUT" 2>&1; echo "=== hash tests exit $? $(date +%T)" >> "$OUT"
nice -n 15 cargo check -p semio-framework-plugin -p semio-framework-plugin-host --tests --message-format short >> "$OUT" 2>&1; echo "=== check exit $? $(date +%T)" >> "$OUT"
nice -n 15 cargo test -p semio-framework-plugin-host --lib --no-fail-fast -- interpreter >> "$OUT" 2>&1; echo "=== host interpreter tests exit $? $(date +%T)" >> "$OUT"
nice -n 15 cargo test -p semio-framework-plugin --lib --no-fail-fast -- app_declarations codec_calls_construct >> "$OUT" 2>&1; echo "=== plugin declaration laws exit $? $(date +%T)" >> "$OUT"
echo "=== done $(date +%T)" >> "$OUT"
