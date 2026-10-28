#!/bin/sh
# usage: verify.sh <root>  runs the six capacity files BARE (no envelope, no artifact directory) and expects the same results as the official suite runner. Needs the td-leases batch `envelope` (declareBuildTestEnvelopeV1) and the td-capacity batches launcherns + custodian applied first.
R=${1:-$(cd "$(dirname "$0")" && while [ ! -d 🧰️framework ] && [ "$PWD" != / ]; do cd ..; done; pwd)}
cd -P "$R" || exit 1
grep -q "export function declareBuildTestEnvelopeV1" 🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts || { echo "MISSING td-leases batch envelope"; exit 2; }
P="🧰️framework/🔨️modules/🏃️process/🏗️build"; status=0
unset SEMIO_SCRIPT_PROCESS_INVOCATION SEMIO_TEST_ARTIFACT_DIR
for f in "$P/🌡️host-capacity/🧪️tests/🧭️liveness" "$P/🎟️capacity/🧪️tests" "$P/🎟️capacity/🧮️units/🧪️tests" "$P/🎟️capacity/🧪️tests/🔐️inheritance" "$P/🎟️capacity/🧪️tests/🪆️scope" "$P/🎟️capacity/🛟️custodian/🧪️tests"; do
  out=$(bun test --timeout 300000 "./$f/🟦️.ts" 2>&1); echo "$(echo "$f" | tail -c 50): $(echo "$out" | /usr/bin/grep -E "^ *[0-9]+ (pass|fail)$" | tr -s " \n" " ")"; echo "$out" | /usr/bin/grep -E "^\(fail\)" | cut -c1-200
  echo "$out" | /usr/bin/grep -qE "^ *0 fail" || status=1
done
exit $status
