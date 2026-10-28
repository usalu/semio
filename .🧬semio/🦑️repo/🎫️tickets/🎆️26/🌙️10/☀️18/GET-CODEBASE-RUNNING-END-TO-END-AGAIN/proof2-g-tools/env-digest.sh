#!/bin/sh
# churn-g envelope laws, one process birth per `repo exec` (the receiver admits one birth per process): env-digest.sh <file-key> <test-name-pattern>
cd "$(dirname "$0")/../../../../../../../.." || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
D="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/🧪️tests"
ART="${PROOF_ART:-/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/p1-art}"; mkdir -p "$ART"
export SEMIO_TEST_ARTIFACT_DIR="$ART" SEMIO_BUILD_GRAPH_POLICY=current
case "$1" in snapshot) F="./$D/🔒️batch-snapshot/🟦️.ts";; digest) F="./$D/🟦️.ts";; esac
exec nice -n 10 sh "$C/proof-g-tools/rp.sh" --tool=semio exec --projects=workspace -- repo tools exec bun -- test "$F" -t "$2"
