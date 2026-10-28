#!/bin/sh
# 🪜 Laws of batch `testorder` after apply: SEMIO_TEST_ARTIFACT_DIR=<dir> sh verify.sh [<repository or mirror root>]
# 1. real-lease order law of the test and Clippy runners (SEMIO_LEASE_ORDER=enforce, Python oracle over the authored policy), repository port AST law (Python text oracle),
#    step-admission law (Python oracle), the updated consumer model and the cpu-last law (needs `git ls-files` in the root);
# 2. the suites in suites.txt: no failure outside known-failures.txt (pre-existing under bare `bun test`: no parent Script invocation; identical on base).
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=${1:-$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")}
out=${SEMIO_TEST_ARTIFACT_DIR:?SEMIO_TEST_ARTIFACT_DIR required}; mkdir -p "$out"
cd "$root"
python3 -I "$here/stage.py" check --root "$root"
bun test ./🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🧪️tests/🪜️order/🟦️.ts ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🧪️tests/🪜️order/🟦️.ts ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/⚙️configuration/🧪️tests/🪜️steps/🟦️.ts ./🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️inputs/🧪️tests/📥️consumers/🟦️.ts ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🧪️tests/⛓️cpu-last/🟦️.ts
bun test --timeout 300000 --reporter=junit --reporter-outfile="$out/testorder-suites.xml" $(tr '\n' ' ' < "$here/suites.txt") > "$out/testorder-suites.out" 2>&1 || true
python3 -I "$here/junit-set.py" "$out/testorder-suites.xml" | grep '^FAIL' | sort > "$out/testorder-suites.set" || true
new=$(comm -13 "$here/known-failures.txt" "$out/testorder-suites.set")
if [ -z "$new" ]; then echo "suites: no failure outside the $(wc -l < "$here/known-failures.txt" | tr -d ' ') pre-existing ones ($(wc -l < "$out/testorder-suites.set" | tr -d ' ') failed now)"; else echo "suites: NEW failures:"; echo "$new"; exit 1; fi
