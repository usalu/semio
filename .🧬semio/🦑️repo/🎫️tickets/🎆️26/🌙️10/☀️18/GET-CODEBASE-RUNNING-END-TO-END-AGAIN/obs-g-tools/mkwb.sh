#!/bin/sh
# [obs-g] mkwb.sh <wb dir>: workbench with the same layout as the real workspace where ONLY the process module, the mcp module and the repo library (minus the 6 GB inputs module) are APFS clones; everything else is a symlink into the real tree. Staged edits are developed and tested there, never in the real tree.
set -e
R=/Users/ueli/Documents/semio; WB="$1"; rm -rf "$WB"; mkdir -p "$WB"
for e in $(ls -A "$R" | /usr/bin/grep -v -E '^(\.git|🧰️framework)$'); do ln -s "$R/$e" "$WB/$e"; done
F="🧰️framework"; mkdir -p "$WB/$F/🔨️modules" "$WB/$F/🛍️products/🦑️repo/🔨️modules"
for e in $(ls -A "$R/$F" | /usr/bin/grep -v -E '^(🔨️modules|🛍️products)$'); do ln -s "$R/$F/$e" "$WB/$F/$e"; done
for e in $(ls -A "$R/$F/🔨️modules" | /usr/bin/grep -v -E '^🏃️process$'); do ln -s "$R/$F/🔨️modules/$e" "$WB/$F/🔨️modules/$e"; done
cp -Rc "$R/$F/🔨️modules/🏃️process" "$WB/$F/🔨️modules/🏃️process"
for e in $(ls -A "$R/$F/🛍️products" | /usr/bin/grep -v -E '^🦑️repo$'); do ln -s "$R/$F/🛍️products/$e" "$WB/$F/🛍️products/$e"; done
P="$F/🛍️products/🦑️repo"
for e in $(ls -A "$R/$P" | /usr/bin/grep -v -E '^🔨️modules$'); do ln -s "$R/$P/$e" "$WB/$P/$e"; done
for e in $(ls -A "$R/$P/🔨️modules" | /usr/bin/grep -v -E '^(📚️library|🔌️mcp)$'); do ln -s "$R/$P/🔨️modules/$e" "$WB/$P/🔨️modules/$e"; done
cp -Rc "$R/$P/🔨️modules/🔌️mcp" "$WB/$P/🔨️modules/🔌️mcp"
LB="$P/🔨️modules/📚️library"; mkdir -p "$WB/$LB/🏗️build"
for e in $(ls -A "$R/$LB" | /usr/bin/grep -v -E '^🏗️build$'); do cp -Rc "$R/$LB/$e" "$WB/$LB/$e"; done
for e in $(ls -A "$R/$LB/🏗️build"); do if [ "$e" = "📥️inputs" ]; then ln -s "$R/$LB/🏗️build/$e" "$WB/$LB/🏗️build/$e"; else cp -Rc "$R/$LB/🏗️build/$e" "$WB/$LB/🏗️build/$e"; fi; done
for f in "📋️project.json" package.json; do rm -f "$WB/$f"; cp "$R/$f" "$WB/$f"; done
echo "$WB"
