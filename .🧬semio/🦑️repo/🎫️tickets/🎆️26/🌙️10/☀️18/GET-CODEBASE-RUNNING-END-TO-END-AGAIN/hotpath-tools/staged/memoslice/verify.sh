#!/bin/sh
# 🍰️ Laws of batch `memoslice` after apply: sh verify.sh <root>   (repository or mirror root)
set -e
here=$(cd "$(dirname "$0")" && pwd); root=${1:?root}
cd "$root"
M=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo
python3 -I "$here/stage.py" check "$root"
bun test "$M/🧪️tests/🍰️slice/🟦️.ts" "$M/🧪️tests/🟦️.ts" ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🧪️tests/🟦️.ts
echo "memoslice laws: GREEN"
