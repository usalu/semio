#!/bin/sh
# 🔒️ Laws of batch `journal` after apply: sh verify.sh <mirror-root>   (a MIRROR root, never the live checkout: the negative control swaps the base files in and out)
ROOT=${1:?mirror root}; HERE=$(cd "$(dirname "$0")" && pwd); XPLAT5=$(cd "$HERE/../.." && pwd); . "$XPLAT5/verify-common.sh"
R='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident'
cd "$ROOT" || exit 2
[ -e "$ROOT/.git" ] && { echo "refusing the live checkout"; exit 2; }
python3 -I "$HERE/stage.py" check --root "$ROOT" || exit 2
law "journal lock decision (Python oracle) and starting-owner lock" bun test "./$R/📜️journal/🧪️tests/🔒️lock/🟦️.ts"
law "journal directory incl. the oracle without pwd/getuid"          bun test "./$R/📜️journal/🧪️tests/📍️directory/🟦️.ts"
law "change journal service and client (unchanged)"                  bun test "./$R/📜️journal/🧪️tests/🟦️.ts"
law "every test file belongs to one suite"                           bun test "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗂️suites/🧪️tests/🟦️.ts"
negative "lock + oracle" "$HERE" "$ROOT" bun test "./$R/📜️journal/🧪️tests/🔒️lock/🟦️.ts" "./$R/📜️journal/🧪️tests/📍️directory/🟦️.ts"
exit $FAILED
