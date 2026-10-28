#!/bin/sh
# 🎛️ verify.sh <root>: apply the batch to a mirror root, the claim law (4) and the existing claim suite (124) pass. FOR a mirror: root = the mirror root (never live).
root=${1:?mirror root}
here=$(cd "$(dirname "$0")" && pwd)
claims="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims"
python3 -I "$here/stage.py" apply --root "$root" || exit 1
cd "$root" || exit 1
bun test "./$claims/🧪️tests/🎛️control/🟦️.ts" && bun test "./$claims/🧪️tests/🟦️.ts"
