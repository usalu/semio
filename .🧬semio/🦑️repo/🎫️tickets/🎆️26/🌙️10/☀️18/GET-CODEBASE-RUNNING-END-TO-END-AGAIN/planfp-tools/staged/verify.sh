#!/bin/sh
# ♻️ Laws of batch `planfp` after apply: VERIFY_ROOT=<repository or mirror root> [SEMIO_TEST_BAZEL_BINARY=<bazel>] sh verify.sh
# The real-Bazel leg of 📥️paths is left out: it fails on base too (path budget vs a long mirror TMPDIR).
# New laws (red on base): witness entry-kind rows; plan-reuse oracle content group + decision cycle (content edit reused, glob add / declarations replan).
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=${VERIFY_ROOT:-$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")}
cd "$root"
B=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build
P=$B/🔧️tools/🏗️bazel
art=${SEMIO_TEST_ARTIFACT_DIR:-$here/../../🗑️generated/planfp/art}
mkdir -p "$art"
python3 -I "$here/stage.py" check --root "$root"
bun test "$B/🔭️witness/🧪️tests/🟦️.ts"
bun test "$P/♻️plan-reuse/🧪️tests/🟦️.ts" "$P/♻️plan-reuse/🧪️tests/🌌️whole/🟦️.ts" "$P/♻️plan-reuse/🧪️tests/🧭️ignore-sources/🟦️.ts"
env -u SEMIO_TEST_BAZEL_BINARY bun test "$P/📸️snapshot/🧪️tests/🟦️.ts" "$P/📸️snapshot/🧪️tests/🔐️mode/🟦️.ts" "$P/🧪️tests/📥️keys/🟦️.ts" "$P/🧪️tests/📥️paths/🟦️.ts" "$P/🎬️scopes/🧪️tests/🧱️workspace/🟦️.ts" "$P/🧪️tests/📥️delegation/🟦️.ts"
bun "$here/classes-check.ts" "$root" | tee /dev/stderr | grep -q "^PASS"
if [ -n "$SEMIO_TEST_BAZEL_BINARY" ]; then
 SEMIO_TEST_ARTIFACT_DIR="$art" bun test --timeout 500000 "$P/🧪️tests/📥️workspace/🟦️.ts" "$P/🏃️execution/🧪️tests/🔒️hermetic/🟦️.ts" "$P/🏃️execution/🧪️tests/🔒️classes/🟦️.ts"
fi
echo "planfp verify: GREEN"
