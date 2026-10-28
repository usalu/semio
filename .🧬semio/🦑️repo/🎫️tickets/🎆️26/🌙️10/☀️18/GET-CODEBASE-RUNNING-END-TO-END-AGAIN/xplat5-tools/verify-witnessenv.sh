#!/bin/sh
# 🖥️ Laws of batch `witnessenv` after apply: sh verify.sh <mirror-root>
ROOT=${1:?mirror root}; HERE=$(cd "$(dirname "$0")" && pwd); XPLAT5=$(cd "$HERE/../.." && pwd); . "$XPLAT5/verify-common.sh"
B='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build'
cd "$ROOT" || exit 2
[ -e "$ROOT/.git" ] && { echo "refusing the live checkout"; exit 2; }
python3 -I "$HERE/stage.py" check --root "$ROOT" || exit 2
law "session variables of every host + Windows process table probe" bun test "./$B/🔭️witness/🧪️tests/🖥️session/🟦️.ts"
law "plan witness contract (unchanged)"                              bun test "./$B/🔭️witness/🧪️tests/🟦️.ts"
law "keyed command and action environment (unchanged)"               bun test $(find "$B/🔧️tools/🏗️bazel/🌿️environment" -path '*🧪️tests*' -name '🟦️.ts' | sed 's#^#./#')
negative "session variables" "$HERE" "$ROOT" bun test "./$B/🔭️witness/🧪️tests/🖥️session/🟦️.ts"
exit $FAILED
