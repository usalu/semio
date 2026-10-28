#!/bin/sh
# usage: sh verify.sh [platforms|freespace|corpus|all]  (graph-free; builds an overlay of the repository with xplat3-g's batch and the batches of this lane staged, then runs single-file tests through the official envelope of store-g-tools/ft.sh; at most one test process at a time)
H="$(cd "$(dirname "$0")" && pwd)"
C="$H/.."
OV="${XPLAT4_OVERLAY:-/private/tmp/claude-501/xplat4-overlay}"
python3 -I "$H/overlay.py" "$OV" "$H/../xplat3-g-tools/staged/files" "$H/staged/platforms/files" "$H/staged/freespace/files" "$H/staged/corpus/files" | tail -1
F="🧰️framework"; P="$F/🔨️modules/🏃️process"; L="$F/🛍️products/🦑️repo/🔨️modules/📚️library"; ST="$P/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/📐️statfs"
platforms() { cat <<EOT
$P/🧪️testing/🗂️suites/🖥️platforms/🧪️tests/🟦️.ts
$P/🧪️testing/🗂️suites/🧪️tests/🟦️.ts
$L/🏗️build/🧾️ci-report/🗂️suites/🧪️tests/🔀️workflow/🟦️.ts
$L/🏗️build/🧾️ci-report/🗂️suites/🧪️tests/🟦️.ts
$L/🏗️build/🧾️ci-report/🧪️tests/🟦️.ts
EOT
}
freespace() { cat <<EOT
$ST/🧪️tests/🆓️unknown/🟦️.ts
$ST/🧪️tests/🟦️.ts
$ST/🧪️tests/🔒️sites/🟦️.ts
$P/🔒️leases/♻️retirement/🧹️sweep/🧪️tests/🟦️.ts
$P/🔒️leases/🚪️admission/🧪️tests/🟦️.ts
$L/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🧪️tests/🟦️.ts
$L/🏗️build/🧹️cache-gc/👯️dedupe/🧪️tests/🟦️.ts
$L/🏗️build/🧹️cache-gc/🧪️tests/🟦️.ts
$L/🏗️build/🧹️cache-gc/🧪️tests/🚀️launch/🟦️.ts
$L/🏗️build/🧹️cache-gc/🧪️tests/💰️budget/🟦️.ts
EOT
}
corpus() { echo "$F/🔨️modules/⚙️runtime/🟢️node/🧲️native/🧪️tests/🟦️.ts"; }
case "${1:-all}" in platforms) LIST=$(platforms);; freespace) LIST=$(freespace);; corpus) LIST=$(corpus);; *) LIST=$(platforms; freespace; corpus);; esac
FAIL=0
echo "$LIST" | while IFS= read -r f; do
  printf '%s\n' "== ${f##*/🔨️modules/}"
  VF_WALL=420 sh "$C/store-g-tools/ft.sh" "$OV/$f" 2>&1 | /usr/bin/grep -E "^ *[0-9]+ (pass|skip|fail)|^\(fail\)|error:" | cut -c1-220
done
