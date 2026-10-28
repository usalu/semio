#!/bin/sh
# 📏 Laws of batch `windows`: sh verify.sh <repository or mirror root>  (needs the official envelope until a declared birth lands: run through td-misc-tools/envrun.sh; base: 2/2, after: 4/0)
set -e
here=$(cd "$(dirname "$0")" && pwd); root=$(cd "${1:?root}" && pwd -P)
python3 -I "$here/stage.py" check --root "$root"
case "$root" in */sm/*) m=$(basename "$root");; *) echo "verify.sh runs the envelope inside a mirror: pass a mirror root"; exit 2;; esac
sh "$here/../../../td-misc-tools/envrun.sh" "$m" @semio-tech/framework-process:test-testing "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧪️tests/📏️windows/🟦️.ts" | grep -E '^\(fail\)|^ *[0-9]+ (pass|fail)$'
