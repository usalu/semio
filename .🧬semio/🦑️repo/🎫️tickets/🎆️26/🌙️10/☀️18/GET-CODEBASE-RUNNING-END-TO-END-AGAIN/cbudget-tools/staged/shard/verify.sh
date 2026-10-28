#!/bin/sh
# 🎓️ Laws of batch `shard` after apply: VERIFY_ROOT=<repository or mirror root> sh verify.sh  (the mirror needs 🎓️teaching/🏛️architecture/📚️course/🧪️tests for the live-config law, a checkout has it)
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=${VERIFY_ROOT:-$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")}
cd "$root"
P=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🚦️test-classes
python3 -I "$here/stage.py" check --root "$root"
bun test "$P/🧪️tests/🎓️course/🟦️.ts" "$P/🧪️tests/🟦️.ts"
