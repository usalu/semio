#!/bin/zsh
# 🔬 Rebuilds the patched scratch copy of the library: clone of the live library + tax4 staged patch (+ tax5 staged patch unless NO_TAX5=1), sibling links and project manifest. usage: [NO_TAX5=1] rebuild-sim.sh [scratch name]
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
NAME="${1:-tax5-sim}"
S="$C/🗑️generated/$NAME/$L"
rm -rf "$S"
mkdir -p "$(dirname "$S")"
cp -cR "$L" "$S" || exit 1
python3 "$C/tax4-tools/staged/apply.py" --root "$S" || exit 1
[ -n "$NO_TAX5" ] || python3 "$C/tax5-tools/staged/apply.py" --root "$S" || exit 1
python3 "$C/tax5-tools/mk-sim-links.py" "$NAME" || exit 1
cp 📋️project.json "$C/🗑️generated/$NAME/📋️project.json"
