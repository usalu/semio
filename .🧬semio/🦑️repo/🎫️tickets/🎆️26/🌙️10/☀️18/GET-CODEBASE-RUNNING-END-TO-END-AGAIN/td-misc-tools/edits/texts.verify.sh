#!/bin/sh
# 📖 Laws of batch `texts`: sh verify.sh <repository or mirror root>  (the process texts law goes green; the suites law keeps only its pre-existing red "every test file of the process module belongs to exactly one suite...", same on base)
set -e
here=$(cd "$(dirname "$0")" && pwd); root=$(cd "${1:?root}" && pwd -P); cd "$root"
python3 -I "$here/stage.py" check --root "$root"
P=./🧰️framework/🔨️modules/🏃️process
bun test "$P/📖️texts/🧪️tests/🟦️.ts"
bun test "$P/🧪️testing/🗂️suites/🧪️tests/🟦️.ts" 2>&1 | grep -E '^\(fail\)|^ *[0-9]+ (pass|fail)$' || true
