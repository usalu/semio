#!/bin/sh
# 🔁 [nxdeadline] (re)builds staged batch `release` from the CURRENT live bytes: copies the 4 edited files into a scratch root, replays the anchored edits (edit_src.py, edit_test.py; exit 1 = anchor lost), adds the new law files and restages with base = live. usage: sh rebuild.sh
T="$(cd "$(dirname "$0")" && pwd)"; C="$(dirname "$T")"; L="$(cd "$C/../../../../../../.." && pwd)"; B="$C/🗑️generated/nxdeadline/build"
E="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement"
N="$E/🔌️nx/📦️graph/🧪️tests/⏱️deadline"
FILES="$E/🔌️nx/🟦️.ts $E/🔌️nx/📦️graph/🟦️.ts $E/📦️budget/📥️opening/🏪️store/🟦️.ts $E/📦️budget/📥️opening/🏪️store/🧪️tests/📦️child/🟦️.ts"
rm -rf "$B"; for f in $FILES; do mkdir -p "$B/$(dirname "$f")"; cp "$L/$f" "$B/$f"; done
python3 -I "$T/edit_src.py" "$B" && python3 -I "$T/edit_test.py" "$B" || exit 1
mkdir -p "$B/$N"; cp "$T/law/🟦️.ts" "$T/law/🔣️.json" "$T/law/🐍️.py" "$B/$N/"
args=""; for f in $FILES; do args="$args $f=live"; done
python3 -I "$T/mkbatch.py" release "$B" $args "$N/🟦️.ts=none" "$N/🔣️.json=none" "$N/🐍️.py=none" && cp "$T/stage.template.py" "$T/staged/release/stage.py"
rm -rf "$B"
