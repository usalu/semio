#!/bin/sh
# 🪪 [td-leases] verify batch `envelope` on <root> (a mirror, or live after apply): the 5 lease suites and the helper's own laws pass under a BARE `bun test ./file` (no parent envelope, no SEMIO_TEST_ARTIFACT_DIR), and the suites coverage law still routes every file. usage: sh verify.sh <root>
R="${1:?root}"; P="$R/🧰️framework/🔨️modules/🏃️process"; rc=0
for d in "🔒️leases/♻️retirement/🧪️tests/📋️claims" "🔒️leases/♻️retirement/🧪️tests/📋️frontier" "🔒️leases/♻️retirement/🧪️tests/📋️inventory" "🔒️leases/♻️retirement/🧹️sweep/🧪️tests" "🔒️leases/🧪️tests/🎟️writer" "🧪️testing/📥️invocation/🧪️tests" "🧪️testing/🗂️suites/🧪️tests"; do
 out=$(cd "$P/$d" && env -u SEMIO_SCRIPT_PROCESS_INVOCATION -u SEMIO_TEST_ARTIFACT_DIR bun test --timeout 170000 ./🟦️.ts 2>&1); s=$?
 sum=$(printf '%s\n' "$out" | grep -E '^ *[0-9]+ (pass|fail)$' | tail -2 | tr -s ' \n' ' ')
 echo "$d:$sum exit $s"; [ $s -eq 0 ] || { rc=1; printf '%s\n' "$out" | grep -E '^\(fail\) ' | tail -5; }
done
[ $rc -eq 0 ] && echo "VERIFY OK envelope" || echo "VERIFY FAILED envelope"; exit $rc
