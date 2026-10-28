#!/bin/sh
# 🗂️ Laws of batch `artdir`: sh verify.sh <repository or mirror root>  (both laws green bare AND with an artifact directory; red bare on base: 4 resp. 2 failures)
set -e
here=$(cd "$(dirname "$0")" && pwd); root=$(cd "${1:?root}" && pwd -P); cd "$root"
python3 -I "$here/stage.py" check --root "$root"
P=./🧰️framework/🔨️modules/🏃️process
for f in "$P/🎛️owned-execution/🧪️tests/🟦️.ts" "$P/🧪️testing/🎛️execution/🧪️tests/🟦️.ts"; do
 env -u SEMIO_TEST_ARTIFACT_DIR bun test "$f"
 a=$(mktemp -d); SEMIO_TEST_ARTIFACT_DIR="$a" bun test "$f"; rm -rf "$a"
done
