#!/bin/sh
set -e
T=$(cd "$(dirname "$0")" && pwd)
OV=${1:-/private/tmp/claude-501/lease-fair-ov}
D="$OV/🧰️framework/🔨️modules/🏃️process/🔒️leases/⚖️fairness"
mkdir -p "$D/🧪️tests/🧫️fixtures" "$D/🧪️tests/📥️worker" "$D/🧬️schema"
cp "$T/fairness-test.ts" "$D/🧪️tests/🟦️.ts"
cp "$T/fairness-fixture.json" "$D/🧪️tests/🧫️fixtures/🔣️.json"
cp "$T/fairness-schema.json" "$D/🧬️schema/🔣️.json"
cp "$T/fairness-worker.ts" "$D/🧪️tests/📥️worker/🟦️.ts"
