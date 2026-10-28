#!/bin/sh
# ✂️ Laws of batch `scopedcut` after apply: sh verify.sh <repository or mirror root>  (default: the root above this batch)
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=${1:-$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")}
cd "$root"
G=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations
python3 -I "$here/stage.py" check --root "$root"
bun test --timeout 120000 "$G/✂️cutoff/🧪️tests/🟦️.ts" "$G/✂️cutoff/🧪️tests/🏪️store/🟦️.ts" "$G/🧪️tests/🟦️.ts"
echo "scopedcut VERIFIED"
