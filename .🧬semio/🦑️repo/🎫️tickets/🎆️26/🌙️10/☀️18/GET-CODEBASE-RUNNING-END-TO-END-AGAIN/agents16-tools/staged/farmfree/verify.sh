#!/bin/sh
# 🔓 Laws of batch `farmfree` after apply: sh verify.sh <repository or mirror root>  (exit 0 = VERIFY OK).
set -e
here=$(cd "$(dirname "$0")" && pwd); root=${1:?root}
cd "$root"
python3 -I "$here/stage.py" check --root "$root"
F=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests
bun test --timeout 120000 "$F/🔓️release/🟦️.ts" "$F/🔒️farm-close/🟦️.ts" "$F/🟦️.ts"
echo "VERIFY OK"
