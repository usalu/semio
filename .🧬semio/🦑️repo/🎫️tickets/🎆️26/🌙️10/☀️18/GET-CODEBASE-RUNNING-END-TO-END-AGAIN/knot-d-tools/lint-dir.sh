#!/bin/sh
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
s=$(date +%s)
SEMIO_TEST_ARTIFACT_DIR="/Users/ueli/Documents/semio/$C/🗑️generated/knot-d/lint-art" SEMIO_LANGUAGE=en SEMIO_BUILD_GRAPH_POLICY=current nice -n 10 bun repo --tool=nx run @semio-tech/repo-lib:lint-dependency-direction > "$C/🗑️generated/knot-d/lint-direction.log" 2>&1
rc=$?
echo "lint-dependency-direction rc=$rc $(( $(date +%s)-s ))s" >> "$C/🗑️generated/knot-d/verdicts.txt"
