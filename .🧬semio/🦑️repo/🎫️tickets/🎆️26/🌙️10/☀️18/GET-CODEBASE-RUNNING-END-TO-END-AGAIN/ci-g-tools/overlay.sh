#!/bin/sh
# usage: overlay.sh -> builds the scratchpad overlay `ov` (symlinks to the real tree, the staged ci-report/suites module as a real copy) so graph-free tests of the staged module run without touching the tree
set -e
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
OV="${CI_G_OVERLAY:-/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/ov}"
R="$PWD"
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
STAGED="$C/ci-g-tools/staged/files"
rm -rf "$OV"; mkdir -p "$OV/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report"
ln -s "$R/node_modules" "$OV/node_modules"; ln -s "$R/package.json" "$OV/package.json"; ln -s "$R/.github" "$OV/.github"
ln -s "$R/🧰️framework/🔨️modules" "$OV/🧰️framework/🔨️modules"; ln -s "$R/🧰️framework/📦️packages" "$OV/🧰️framework/📦️packages"
for d in "$R/🧰️framework/"*; do b=$(basename "$d"); [ -e "$OV/🧰️framework/$b" ] || ln -s "$d" "$OV/🧰️framework/$b"; done
for d in "$R/🧰️framework/🛍️products/🦑️repo/"*; do b=$(basename "$d"); [ "$b" = "🔨️modules" ] && continue; ln -s "$d" "$OV/🧰️framework/🛍️products/🦑️repo/$b"; done 2>/dev/null || true
mkdir -p "$OV/🧰️framework/🛍️products/🦑️repo/🔨️modules"
for d in "$R/$LIB/../"*; do b=$(basename "$d"); [ "$b" = "📚️library" ] && continue; [ -e "$OV/🧰️framework/🛍️products/🦑️repo/🔨️modules/$b" ] || ln -s "$d" "$OV/🧰️framework/🛍️products/🦑️repo/🔨️modules/$b"; done
for d in "$R/$LIB/"*; do b=$(basename "$d"); [ "$b" = "🏗️build" ] && continue; ln -s "$d" "$OV/$LIB/$b"; done
for d in "$R/$LIB/🏗️build/"*; do b=$(basename "$d"); [ "$b" = "🧾️ci-report" ] && continue; ln -s "$d" "$OV/$LIB/🏗️build/$b"; done
for d in "$R/$LIB/🏗️build/🧾️ci-report/"*; do b=$(basename "$d"); ln -s "$d" "$OV/$LIB/🏗️build/🧾️ci-report/$b"; done
cp -R "$STAGED/$LIB/🏗️build/🧾️ci-report/🗂️suites" "$OV/$LIB/🏗️build/🧾️ci-report/🗂️suites"
echo "$OV"
