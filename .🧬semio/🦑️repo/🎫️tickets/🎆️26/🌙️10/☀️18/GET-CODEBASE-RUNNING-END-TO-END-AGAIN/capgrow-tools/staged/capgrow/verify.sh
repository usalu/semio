#!/bin/sh
# 🧪️ capgrow laws on a root (a mirror with the batch applied, or live after landing): oracles, every touched suite one file at a time; exit 1 on a failure outside the known invocation-envelope set.
R=${1:?root}
C="$(cd "$(dirname "$0")/../../.." && pwd)"
export SEMIO_TEST_ARTIFACT_DIR="${SEMIO_TEST_ARTIFACT_DIR:-$C/🗑️generated/capgrow/verify}"; mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
out=$(sh "$(dirname "$0")/suites.sh" "$R" 2>&1); printf '%s\n' "$out"
known='real core leases admit the planned compiles|a compile waiting for its minimum cores is cancelled|Bun bundlers are admitted side by side|the actual inherited gate branch refuses stale boot|isolated Cargo stores are removed on every owner settlement|\(unnamed\)'
bad=$(printf '%s\n' "$out" | grep -E '^ +\(fail\)' | grep -Ev "$known"; printf '%s\n' "$out" | grep -E '^oracle' | grep -v 'vectors equal the oracle')
if [ -n "$bad" ]; then echo "VERIFY FAIL:"; printf '%s\n' "$bad"; exit 1; fi
echo "VERIFY OK (failures only in the known invocation-envelope set, identical on base)"
