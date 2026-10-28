#!/bin/sh
# 🛑 laws of batch `sigexit` (lane audit12): sh verify.sh <root>  (after apply). Red on live base: digest 3/3 and window 2/2 (children exit 130 before they print DATA); green: digest 5/0, window 3/0; the sigint law of the watchdog suite keeps 12/0 (exit 130/143, one notice, no raw dump; Python subprocess oracle). On a mirror run it unsealed (`mirror.py run <m> --no-seal`): the macOS seal changes process liveness and two ps-based watchdog laws fail under it on base too.
ROOT="$(cd "${1:?root}" && pwd -P)"; HERE="$(cd "$(dirname "$0")" && pwd)"; . "$HERE/../../verify.common.sh"; applied
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs"
law 12/0 "🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🧪️tests/🟦️.ts"
law 24/0 "🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🟦️.ts"
LAWENV="-u SEMIO_TEST_ARTIFACT_DIR" law 5/0 "$L/🧮️digest/🧪️tests/🟦️.ts"
LAWENV="-u SEMIO_TEST_ARTIFACT_DIR" law 3/0 "$L/🧪️tests/📏️windows/🟦️.ts"
parse; mode
[ $FAILED = 0 ] && echo "VERIFY OK sigexit" || echo "VERIFY FAIL sigexit"; exit $FAILED
