#!/bin/sh
# 🧒 Laws of batch `childheap` after apply: VERIFY_ROOT=<repository or mirror root> sh verify.sh  (default: the root above this batch)
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=${VERIFY_ROOT:-$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")}
cd "$root"
B=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build
python3 -I "$here/stage.py" check --root "$root"
if rg -n -g '!**/🧪️tests/**' "buildGraphChildHeapBytesV1" "$B" ; then echo "stale reference"; exit 1; fi
bun test "$B/🧠️memory/🧪️tests/🟦️.ts" "$B/🧭️invocation/🧪️tests/🧊️isolated-plan/🟦️.ts"
