#!/bin/sh
# 🕳️ Laws of batch `planfp` round 3 after apply: VERIFY_ROOT=<repository or mirror root> sh verify.sh
# Red on live: plan-reuse oracle "a build writing its own ignored store" (d …/course/.store changed); "new source file beside the project's ignored store" must still replan.
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=${VERIFY_ROOT:-$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")}
cd "$root"
P=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel
python3 -I "$here/stage.py" check --root "$root"
bun test "$P/♻️plan-reuse/🧪️tests/🟦️.ts" "$P/♻️plan-reuse/🧪️tests/🌌️whole/🟦️.ts" "$P/♻️plan-reuse/🧪️tests/🧭️ignore-sources/🟦️.ts" "$P/🎬️scopes/🧪️tests/🧱️workspace/🟦️.ts" "$P/🧪️tests/📥️keys/🟦️.ts" "$P/🧪️tests/📥️fingerprint/🟦️.ts"
echo "planfp staged3 verify: GREEN"
