#!/bin/sh
# 🪆 Laws of batch `orchslice` after apply: sh verify.sh <repository or mirror root>  (default: the root above this batch)
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=${1:-$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")}
cd "$root"
G=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph
python3 -I "$here/stage.py" check --root "$root"
python3 -I "$G/🕰️generations/🔌️load/🧪️tests/🐍️.py" "$G/🕰️generations/🔌️load/🧪️tests/🔣️.json"
bun test --timeout 120000 "$G/🕰️generations/🔌️load/🧪️tests/🟦️.ts" "$G/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🟦️.ts" "$G/🕰️generations/🧷️pin/🧪️tests/🟦️.ts" "$G/🕰️generations/🧷️pin/🧪️tests/🤝️nested/🟦️.ts"
echo "orchslice VERIFIED"
