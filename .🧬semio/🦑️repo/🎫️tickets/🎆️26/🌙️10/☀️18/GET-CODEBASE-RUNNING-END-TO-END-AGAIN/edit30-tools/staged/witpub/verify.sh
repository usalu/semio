#!/bin/sh
# 🧾 Laws of batch `witpub`: sh verify.sh <repository or mirror root>   (real Bazel laws use SEMIO_TEST_BAZEL_BINARY, default: the pinned binary of the live cache)
# 1. the witness laws (publisher digest = reference = Python hashlib; plan closed; real Bazel: a witnessed test reruns exactly when the published witness moved);
# 2. every suite of the Bazel tool: the failure set must be a subset of the recorded base set (`base.set`, pre-existing harness/envelope failures).
set -e
here=$(cd "$(dirname "$0")" && pwd); root=${1:?root}; out="$here/../../../🗑️generated/edit30"; mkdir -p "$out"
export SEMIO_TEST_BAZEL_BINARY=${SEMIO_TEST_BAZEL_BINARY:-$(ls /Users/ueli/Documents/semio/.cache/b/t/m/bazel/bin/*/bazel 2>/dev/null | head -1)}
python3 -I "$here/stage.py" check --root "$root"
Z=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel
cd "$root" && bun test "$Z/🚦️test-classes/🧾️witness/🧪️tests/🟦️.ts" "$Z/🚦️test-classes/🧾️witness/🧪️tests/🧮️digest/🟦️.ts" "$Z/🧪️tests/📥️coverage/🟦️.ts"
sh "$here/../../suites.sh" "$root" "$here/suites.txt" wp-verify
python3 -I -c 'import sys
base={l for l in open(sys.argv[1]) if l.startswith("FAIL")};now={l for l in open(sys.argv[2]) if l.startswith("FAIL")}
new=sorted(now-base)
print("new failures:",len(new));[print(" ",l.rstrip()) for l in new]
sys.exit(1 if new else 0)' "$here/base.set" "$out/wp-verify.set"
echo "VERIFY OK"
