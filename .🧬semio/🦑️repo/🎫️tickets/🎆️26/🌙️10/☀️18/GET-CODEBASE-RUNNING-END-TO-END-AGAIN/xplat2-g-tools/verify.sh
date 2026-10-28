#!/bin/sh
# Single-file graph-free runs (official envelope, store-g-tools/ft.sh) of every xplat2-g test and of the neighbours of every edited file.
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE" && git rev-parse --show-toplevel) || exit 1
cd "$ROOT" || exit 1
P=🧰️framework/🔨️modules/🏃️process
L=$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library
S=$P/📦️artifacts/🏗️native-build/💾️storage
FAIL=0
run() { printf '== %s: ' "$(echo "$1" | awk -F/ '{print $(NF-2)"/"$(NF-1)}')"; OUT=$(sh "$HERE/../store-g-tools/ft.sh" "$1" 2>&1); echo "$OUT" | grep -E "^ *[0-9]+ (pass|fail)" | tr '\n' ' '; echo; echo "$OUT" | grep -q "^ *0 fail" || { echo "$OUT" | grep -E "\(fail\)|error:" | head -5; FAIL=1; }; }
for f in \
  🔒️leases/📁️physical/🧪️tests/🪟️windows/🟦️.ts 🔒️leases/📁️physical/🧪️tests/📂️descriptors/🟦️.ts \
  🔒️leases/🪟️windows/🧪️tests/🟦️.ts 🔒️leases/🧭️order/🧪️tests/🪟️registry-root/🟦️.ts \
  🪓️termination/🧪️tests/🟦️.ts 🪓️termination/🧪️tests/🪟️taskkill/🟦️.ts \
  📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🩹️recovery/🕰️clock/🪟️native/🧪️tests/🟦️.ts \
  📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🩹️recovery/🕰️clock/🧪️tests/🟦️.ts \
  📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🩹️recovery/🧪️tests/🟦️.ts ; do run "$f"; done
for f in \
  "$L/🏗️build/🩺️preflight/💊️remedies/🧪️tests/🟦️.ts" "$L/🏗️build/🩺️preflight/🧪️tests/🟦️.ts" "$L/🏗️build/🩺️preflight/🧪️tests/🧮️matrix/🟦️.ts" \
  "$L/🏃️process/🦀️cargo/🔣️policy/🧪️tests/🪟️nextest/🟦️.ts" "$L/🏃️process/🦀️cargo/🔣️policy/🧪️tests/🟦️.ts" \
  "$L/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🪟️taskkill/🟦️.ts" "$L/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🪟️shim/🟦️.ts" "$L/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/👷️worker/🟦️.ts" \
  "$L/🏗️build/🧭️selection/🧪️tests/🟦️.ts" ; do run "$f"; done
echo "== suite leases"; VF_WALL=900 bun "$HERE/../ver-f-tools/proc-run.ts" suite leases > "$HERE/../🗑️generated/xplat2-g/leases-suite-final.log" 2>&1 && echo pass || { echo FAIL; FAIL=1; }
exit $FAIL
