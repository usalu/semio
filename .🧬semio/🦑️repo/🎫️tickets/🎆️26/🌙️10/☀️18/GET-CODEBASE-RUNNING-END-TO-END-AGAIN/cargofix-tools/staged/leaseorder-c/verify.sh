#!/bin/sh
# ✍ Laws of batch `leaseorder-c` after apply: VERIFY_ROOT=<repository or mirror root> SEMIO_TEST_ARTIFACT_DIR=<dir> sh verify.sh
# 1. writer-order law (real leases, SEMIO_LEASE_ORDER=enforce, Python oracle over the authored policy); 2. source-order law (TypeScript AST + Python text oracle);
# 3. existing native-build / component-pass / wasm / dependencies / capacity / lease-order / fairness / admission / unit-cache suites: no failure outside known-failures.txt (pre-existing under bare `bun test`, identical on base).
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=${VERIFY_ROOT:-$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")}
out=${SEMIO_TEST_ARTIFACT_DIR:?SEMIO_TEST_ARTIFACT_DIR required}; mkdir -p "$out"
cd "$root"
python3 -I "$here/stage.py" check --root "$root"
bun test ./🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️inputs/🧪️tests/✍️writer/🟦️.ts ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🧪️tests/✍️writer/🟦️.ts
bun test --timeout 300000 --reporter=junit --reporter-outfile="$out/leaseorder-suites.xml" $(tr '\n' ' ' < "$here/suites.txt") > "$out/leaseorder-suites.out" 2>&1 || true
python3 -I "$here/junit-set.py" "$out/leaseorder-suites.xml" | grep '^FAIL' | sort > "$out/leaseorder-suites.set" || true
new=$(comm -13 "$here/known-failures.txt" "$out/leaseorder-suites.set")
if [ -z "$new" ]; then echo "suites: no failure outside the $(wc -l < "$here/known-failures.txt" | tr -d ' ') pre-existing ones ($(wc -l < "$out/leaseorder-suites.set" | tr -d ' ') failed now)"; else echo "suites: NEW failures:"; echo "$new"; exit 1; fi
