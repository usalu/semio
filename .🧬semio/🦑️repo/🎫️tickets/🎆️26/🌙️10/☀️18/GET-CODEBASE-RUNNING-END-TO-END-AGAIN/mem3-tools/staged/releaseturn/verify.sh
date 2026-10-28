#!/bin/sh
# 🧎️ Laws of batch `releaseturn` after apply: sh verify.sh <repository or mirror root>. BUN=<bun> overrides the runtime.
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=${1:-$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")}
BUN=${BUN:-$(ls -d "$root"/.cache/b/t/m/bun/*/bun 2>/dev/null | head -1)}
BUN=${BUN:-bun}
cd "$root"
python3 -I "$here/stage.py" check --root "$root"
L=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build
"$BUN" test "$L/🧠️memory/🧪️tests/🧎️release-turn/🟦️.ts" "$L/🧠️memory/🧪️tests/🟦️.ts" "$L/🕸️graph/🕰️generations/🔌️load/🧪️tests/🕊️early-release/🟦️.ts"
