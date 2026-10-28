#!/bin/sh
# 🕊 Laws of batch `releasefirst` after apply: VERIFY_ROOT=<repository or mirror root> sh verify.sh  (default: the root above this batch)
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=${VERIFY_ROOT:-$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")}
cd "$root"
G=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔌️load
python3 -I "$here/stage.py" check --root "$root"
bun test "$G/🧪️tests/🕊️early-release/🟦️.ts" "$G/🧪️tests/🟦️.ts" "$G/🧪️tests/🪑idle-builder/🟦️.ts" "$G/🧪️tests/🚧️withheld-dependency/🟦️.ts"
