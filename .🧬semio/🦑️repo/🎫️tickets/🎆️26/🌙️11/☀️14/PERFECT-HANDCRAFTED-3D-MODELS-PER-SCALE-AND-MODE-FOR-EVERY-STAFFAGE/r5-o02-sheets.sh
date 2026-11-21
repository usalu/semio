#!/bin/sh
# 🍲 Round 5 O02: clears this asset's old r3 renders, then renders every sheet of the brief via r5-sheets.sh into 🗑️generated/r5-<slug>/<tag>/. Usage: sh r5-o02-sheets.sh <category/slug> <tag> "<pose ids>" "<clip id> <t>"
cd "$(dirname "$0")" || exit 1
A="$1"; SLUG="${A#*/}"
rm -f 🗑️generated/r3-sheet/${A%/*}/$SLUG.* 🗑️generated/r3-2d/${A%/*}/$SLUG.*
rm -rf "🗑️generated/r5-$SLUG/$2"
sh r5-sheets.sh "$@"
