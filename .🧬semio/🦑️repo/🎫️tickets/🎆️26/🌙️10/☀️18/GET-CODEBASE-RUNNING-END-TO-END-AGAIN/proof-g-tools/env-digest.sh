#!/bin/sh
# The digest laws that need the repository envelope (new batch-snapshot law + the existing exact-source laws), through the real `repo exec`.
cd "$(dirname "$0")/../../../../../../../.."
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
D="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/🧪️tests"
export SEMIO_TEST_ARTIFACT_DIR="${PROOF_ART:-/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/p1-art}"
export SEMIO_BUILD_GRAPH_POLICY=current
exec nice -n 10 sh "$C/proof-g-tools/rp.sh" --tool=semio exec --projects=workspace -- repo tools exec bun -- test "./$D/🔒️batch-snapshot/🟦️.ts" "./$D/🟦️.ts"
