#!/bin/zsh
# 🔬 Rebuilds the patched scratch copy of the library: real copy of the live library + the tax6 staged patch, sibling links and project manifest. usage: sim-rebuild.sh [scratch name]
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
NAME="${1:-tax6-sim}"
S="$C/🗑️generated/$NAME/$L"
[ -d "$S" ] && [ ! -L "$S" ] && rm -rf "$S"
mkdir -p "$(dirname "$S")"
cp -cR "$L" "$S" || exit 1
python3 "$C/tax6-tools/staged/apply.py" --root "$S" || exit 1
python3 "$C/tax6-tools/mk-sim-links.py" "$NAME" || exit 1
cp 📋️project.json "$C/🗑️generated/$NAME/📋️project.json"
