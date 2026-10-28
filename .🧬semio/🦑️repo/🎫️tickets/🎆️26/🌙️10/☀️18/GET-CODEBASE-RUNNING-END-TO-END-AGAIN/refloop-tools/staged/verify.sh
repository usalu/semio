#!/bin/sh
# 🔁 Laws of batch `refloop` after apply: sh verify.sh [ROOT]  (default: VERIFY_ROOT or the root above this batch). RED_ON_BASE=1 restores the 3 base sources around the law run (it must fail), then re-applies them.
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=${1:-${VERIFY_ROOT:-$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")}}
cd "$root"
L=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library
D=$L/🕸️dependencies/🟦️typescript
B=$L/🏗️build/🕸️graph/📥️inference/🔁️bootstrap
export SEMIO_TEST_ARTIFACT_DIR=${SEMIO_TEST_ARTIFACT_DIR:-$root/.tmp/refloop-art}
python3 -I "$here/stage.py" check "$root"
if [ "${RED_ON_BASE:-0}" = 1 ]; then
 cp "$here/base/00.mjs" "$D/🟨️.mjs"; cp "$here/base/01.d.mts" "$D/🟨️.d.mts"; cp "$here/base/02.mjs" "$B/🟨️.mjs"
 status=0; bun test "$D/🧪️tests/🔁️verifier/🟦️.ts" || status=$?
 cp "$here/files/00.mjs" "$D/🟨️.mjs"; cp "$here/files/01.d.mts" "$D/🟨️.d.mts"; cp "$here/files/02.mjs" "$B/🟨️.mjs"
 python3 -I "$here/stage.py" check "$root"
 if [ "$status" = 0 ]; then echo "RED_ON_BASE: FAILED (the law passes without the fix)"; exit 1; fi
 echo "RED_ON_BASE: ok (the law fails without the fix)"; exit 0
fi
bun test --timeout 120000 "$D/🧪️tests/🔁️verifier/🟦️.ts" "$L/🏗️build/🕸️graph/📥️inference/🔖️stamps/🧪️tests/🟦️.ts"
