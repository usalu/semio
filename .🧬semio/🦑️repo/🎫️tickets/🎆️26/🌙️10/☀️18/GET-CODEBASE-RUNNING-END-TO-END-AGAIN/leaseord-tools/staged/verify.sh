#!/bin/sh
# Laws of the bazel-picker lock class (run AFTER apply; arg 1 = root, default: walk up from this script to the directory holding 🧰️framework). Order suite (new leaf-lock law), sessions suite (new production-path law: the picker is granted under a held graph snapshot pin with the lock order ENFORCED), the other lease-order suites, fairness and admission consumers of the authored policy. (The capacity and bundle-concurrency suites need the official `repo test` runner envelope and fail identically on base under bare bun test; they are not part of this check.)
R=${1:-$(cd "$(dirname "$0")" && while [ ! -d 🧰️framework ] && [ "$PWD" != / ]; do cd ..; done; pwd)}
cd "$R" || exit 1
F="🧰️framework"; P="$F/🔨️modules/🏃️process"; L="$F/🛍️products/🦑️repo/🔨️modules/📚️library"
status=0
export SEMIO_TEST_ARTIFACT_DIR=${SEMIO_TEST_ARTIFACT_DIR:-$R/.tmp/leaseord-art}; mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
for file in "$P/🔒️leases/🧭️order/🧪️tests/🟦️.ts" "$L/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🧪️tests/🟦️.ts" "$P/🔒️leases/🧭️order/🧪️tests/📥️processes/🟦️.ts" "$P/🔒️leases/🧭️order/🧪️tests/🧩️component-driver/🟦️.ts" "$P/🔒️leases/🧭️order/🧪️tests/🕸️wait-for/🟦️.ts" "$P/🔒️leases/⚖️fairness/🧪️tests/🟦️.ts" "$P/🔒️leases/🚪️admission/🧪️tests/🟦️.ts"; do
  [ -f "$file" ] || { echo "MISSING $file"; status=1; continue; }
  out=$(bun test --timeout 300000 "./$file" 2>&1); echo "$out" | /usr/bin/grep -E "^\(fail\)|^ *[0-9]+ (pass|fail)"
  echo "$out" | /usr/bin/grep -qE "^ *0 fail" || status=1
done
exit $status
