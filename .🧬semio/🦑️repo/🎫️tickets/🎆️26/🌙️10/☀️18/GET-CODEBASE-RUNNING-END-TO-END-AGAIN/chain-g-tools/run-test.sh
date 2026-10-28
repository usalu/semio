#!/bin/sh
# Runs one explicit bun test file under a one-hour Original envelope with the real bootstrap capabilities and a private real-path artifact directory (for laws that use withBuildTestInvocationV1).
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
ART=/private/tmp/chain-g-art; mkdir -p "$ART"
SEMIO_TEST_ARTIFACT_DIR="$ART" SEMIO_SCRIPT_PROCESS_INVOCATION="$(bun "$C/chain-g-tools/wire.ts")" exec bun test "$@"
