#!/bin/sh
# 🗂 Laws of batch `gitrow` after apply: VERIFY_ROOT=<repository or mirror root> sh verify.sh   (default: the root above this batch)
# RED_ON_BASE=1 (mirror only): restores the base script.ts, expects the whole-universe law to fail (needless replan after a staged ticket file), then re-applies.
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=${VERIFY_ROOT:-$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")}
cd "$root"
P=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel
python3 -I "$here/stage.py" check --root "$root"
if [ "${RED_ON_BASE:-0}" = 1 ]; then
 cp "$here/base/00.ts" "$P/📜️script.ts"
 if bun test --timeout 300000 "$P/♻️plan-reuse/🧪️tests/🌌️whole/🟦️.ts"; then cp "$here/files/00.ts" "$P/📜️script.ts"; echo "RED_ON_BASE: the law passed on base (it does not bite)"; exit 1; fi
 echo "RED_ON_BASE: the law fails on base, as required"
 cp "$here/files/00.ts" "$P/📜️script.ts"
fi
bun test --timeout 300000 "$P/♻️plan-reuse/🧪️tests/🌌️whole/🟦️.ts" "$P/♻️plan-reuse/🧪️tests/🟦️.ts" "$P/♻️plan-reuse/🧪️tests/🧭️ignore-sources/🟦️.ts" "$P/🌌️universe/🧪️tests/🟦️.ts"
