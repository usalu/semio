#!/usr/bin/env bash
# 🧪 Runs every verification of the langs-conc set against an overlay, one process at a time at nice 10. Usage: run-all.sh <overlay> [baseline-overlay]. Output: one summary line per suite; details under /private/tmp/langs-conc/results.
set -u
OV="${1:-/private/tmp/langs-conc/ov}"; RESULTS=/private/tmp/langs-conc/results; mkdir -p "$RESULTS"
HERE="$(cd "$(dirname "$0")" && pwd)"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; B="$L/🏗️build"; BZ="$B/🔧️tools/🏗️bazel"; P="🧰️framework/🔨️modules/🏃️process"
for name in $(env | sed -n "s/^\(GIT_[A-Z_]*\)=.*/\1/p"); do unset "$name"; done
cd "$OV" || exit 1
calm() { while [ "$(sysctl -n vm.loadavg | tr -d "{}" | awk "{print int(\$1)}")" -ge 30 ]; do sleep 60; done; }
suite() {
  local name="$1"; shift
  calm
  local out="$RESULTS/$name.log"
  LC_ROOT="$OV" nice -n 10 bun test "$@" >"$out" 2>&1
  printf '%-34s %s\n' "$name" "$(/usr/bin/grep -E '^ *[0-9]+ (pass|fail)$' "$out" | tr -s ' \n' ' ')"
}
suite toolchains-contract "./$P/🏗️build/🧰️toolchains/🧪️tests/🟦️.ts"
suite native-provisioning "./$L/⚡️caching/🚀️bootstrap/🛠️tools/🧰️native/🧪️tests/🟦️.ts"
suite bazel-native-classes "./$BZ/🧰️toolchains/🧪️tests/🟦️.ts"
suite bazel-go "./$BZ/🐹️go/🧪️tests/🟦️.ts"
suite bazel-fingerprint "./$BZ/🧪️tests/📥️fingerprint/🟦️.ts"
suite bazel-cargo "./$BZ/🦀️cargo/🧪️tests/🟦️.ts"
suite bazel-closure "./$BZ/🧪️tests/📥️closure/🟦️.ts"
suite hermetic-table "./$B/🏃️execution/🔒️hermetic/🧪️tests/🟦️.ts"
suite bazel-execution "./$BZ/🏃️execution/🧪️tests/🟦️.ts"
suite inputs-inference "./$B/📥️inputs/🧪️tests/🟦️.ts"
suite pins-law "./$L/🧪️tests/🧰️toolchain-pins/🟦️.ts"
suite cross-platform-bootstrap "./$L/🧪️tests/🥾️cross-platform-bootstrap/🟦️.ts"
suite styling-python-builder "./$L/🎨️styling/🏗️builder/🐍️python/🧪️tests/🟦️.ts"
suite cmake-driver "./$B/🛠️cmake/🧪️tests/🟦️.ts"
suite lab-native-dependencies "$HERE/deps-lab.test.ts"
suite lab-energy-oracle "$HERE/oracle-lab.test.ts"
"$HERE/uv-builder-lab.sh" /private/tmp/langs-conc/builder-lab "$OV/🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🐍️python" | /usr/bin/grep -E "sync\(managed\)|python \(" | sed 's/^/lab-uv-builder                    /'
