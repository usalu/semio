#!/bin/sh
# 📜️ Laws of batch `oncecapture` after apply: sh verify.sh <root>   (repository or mirror root). The integration files at the end need the envelope of the repo test runner; in a bare `bun test` they fail identically before and after the batch ("Original parent Script process invocation required"), so they are reported, not gated.
set -e
here=$(cd "$(dirname "$0")" && pwd); root=${1:?root}
cd "$root"
P=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build
python3 -I "$here/stage.py" check "$root"
bun test "$P/📥️source/🔏️closure/🧪️tests/📜️text/🟦️.ts" "$P/📥️source/🔏️closure/🧪️tests/🧱️static/🟦️.ts"
echo "oncecapture laws: GREEN"
for f in "$P/📥️source/🔏️closure/🚪️command/🧪️tests/🟦️.ts" "$P/⌨️command/🧪️tests/🟦️.ts" "$P/🧭️invocation/🧪️tests/🔗️process-chain/🟦️.ts"; do echo "integration (needs the test-runner envelope): $f"; bun test "$f" 2>&1 | grep -E "^ *[0-9]+ (pass|fail)$" || true; done
