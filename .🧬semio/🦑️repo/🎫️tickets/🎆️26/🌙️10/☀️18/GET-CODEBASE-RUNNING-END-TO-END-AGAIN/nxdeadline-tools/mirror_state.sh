#!/bin/sh
# 🔁 [nxdeadline] puts the three source files of batch `release` in a mirror into state base (live bytes; nxsparse landed on live 10-27 evening) or fix (base + edit_src.py): mirror_state.sh <mirror-root> base|fix
R="$1"; S="$2"; C="$(cd "$(dirname "$0")/.." && pwd)"; L=/Users/ueli/Documents/semio
E="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement"
cp "$L/$E/🔌️nx/🟦️.ts" "$R/$E/🔌️nx/🟦️.ts" && cp "$L/$E/📦️budget/📥️opening/🏪️store/🟦️.ts" "$R/$E/📦️budget/📥️opening/🏪️store/🟦️.ts" && cp "$L/$E/🔌️nx/📦️graph/🟦️.ts" "$R/$E/🔌️nx/📦️graph/🟦️.ts" && cp "$L/$E/📦️budget/📥️opening/🏪️store/🧪️tests/📦️child/🟦️.ts" "$R/$E/📦️budget/📥️opening/🏪️store/🧪️tests/📦️child/🟦️.ts" || exit 2
[ "$S" = fix ] && { python3 -I "$C/nxdeadline-tools/edit_src.py" "$R" && python3 -I "$C/nxdeadline-tools/edit_test.py" "$R" || exit 1; }
echo "state $S"
