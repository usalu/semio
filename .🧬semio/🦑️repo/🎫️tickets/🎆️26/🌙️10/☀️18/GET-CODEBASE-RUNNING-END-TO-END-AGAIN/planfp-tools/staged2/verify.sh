#!/bin/sh
# 🔁 Laws of batch `planfp` round 2 after apply: VERIFY_ROOT=<repository or mirror root> sh verify.sh
# Red on live (planfp landed): 📥️fingerprint 0/4, 📥️inventory 0/2 (constant lookup, older), 🐹️go and 🧰️toolchains 1 fail each, 🔏️implementation "bind immutable" fails.
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=${VERIFY_ROOT:-$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")}
cd "$root"
P=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel
art=${SEMIO_TEST_ARTIFACT_DIR:-$here/../../🗑️generated/planfp/art}
mkdir -p "$art"
python3 -I "$here/stage.py" check --root "$root"
SEMIO_TEST_ARTIFACT_DIR="$art" bun test "$P/🧪️tests/📥️fingerprint/🟦️.ts" "$P/🧪️tests/📥️inventory/🟦️.ts" "$P/🐹️go/🧪️tests/🟦️.ts" "$P/🧰️toolchains/🧪️tests/🟦️.ts"
bun test "$P/🧪️tests/🔏️implementation/🟦️.ts" -t "bind immutable"
bun test "$P/♻️plan-reuse/🧪️tests/🟦️.ts" "$P/🎬️scopes/🧪️tests/🧱️workspace/🟦️.ts" "$P/🧪️tests/📥️keys/🟦️.ts"
echo "planfp staged2 verify: GREEN"
