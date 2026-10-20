#!/bin/sh
# Post-GO runbook of the conc-nx set (run by the executor after the coordinator's GO and a green chain15). Foreground, one step at a time.
set -e
T=$(cd "$(dirname "$0")" && pwd); ROOT=$(cd "$T/../../../../../../.." && pwd)
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
cd "$T"
python3 ops_gen.py                                   # re-base anchors on the live files (aborts when an anchor moved)
python3 apply.py check                               # group 1: 14 ops, group 2: 11 ops (3 put, 8 daemon), 0 drift expected
python3 apply.py apply --write --group 1             # refuses while a repo run / nx process is active
python3 apply.py apply --write --group 2-daemon      # independent of group 1
# group 2-put needs conc-cargo group lease (withCompilerSharedHeadroomV1 in the capacity module); apply.py reports BLOCKED until then:
#   python3 apply.py apply --write --group 2-put
cd "$ROOT"
export SEMIO_TEST_ARTIFACT_DIR="$T/../🗑️generated/conc-nx/art"; mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
bun test "./$L/⚡️caching/🕸️graph/🔏️seal/📌️generation/🧪️tests/🟦️.ts"
bun test "./$L/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🔓️pool-order/🟦️.ts"
bun test "./$L/⚡️caching/🕸️graph/🏁️startup/🚏️admission/🧪️tests/🟦️.ts"
bun test "./$L/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/🎟️put-slots/🧪️tests/🟦️.ts"   # only after group 2-put
# the authorizer closure contains both edited enforcement files: recompile it
bun repo tools setup nx
# real-image laws of the pool (one process per test, see C/nxpart-tools/run-test.ts): funded birth/retire, store, operation
#   bun C/nxpart-tools/run-test.ts "$L/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🏦️funded/🟦️.ts"
# measurement: first run builds + writes the receipt, second run must print "Graph observation selected=... sealed=true"
#   SEMIO_BUILD_PROFILE_INPUTS=1 bun repo run <small project>:<cached target>   (twice, then with a second shell at the same time)
echo "conc-nx live steps done; delete $T/../🗑️generated/conc-nx when finished"
