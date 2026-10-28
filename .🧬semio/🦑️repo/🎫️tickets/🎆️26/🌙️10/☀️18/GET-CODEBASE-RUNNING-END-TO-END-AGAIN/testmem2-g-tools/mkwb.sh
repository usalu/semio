#!/bin/sh
# mkwb.sh <wb dir>: workbench = real workspace layout where ONLY the process module is a private clone (APFS clonefile), everything else a symlink into the real tree; staged edits are tested there, never in the tree.
set -e
R=/Users/ueli/Documents/semio; WB="$1"; rm -rf "$WB"; mkdir -p "$WB"
for e in $(ls -A "$R" | /usr/bin/grep -v -E '^(\.git|\.cache|🧰️framework)$'); do ln -s "$R/$e" "$WB/$e"; done
ln -s "$R/.cache" "$WB/.cache"
mkdir -p "$WB/🧰️framework/🔨️modules"
for e in $(ls -A "$R/🧰️framework" | /usr/bin/grep -v -E '^🔨️modules$'); do ln -s "$R/🧰️framework/$e" "$WB/🧰️framework/$e"; done
for e in $(ls -A "$R/🧰️framework/🔨️modules" | /usr/bin/grep -v -E '^🏃️process$'); do ln -s "$R/🧰️framework/🔨️modules/$e" "$WB/🧰️framework/🔨️modules/$e"; done
cp -Rc "$R/🧰️framework/🔨️modules/🏃️process" "$WB/🧰️framework/🔨️modules/🏃️process"
echo "$WB"
