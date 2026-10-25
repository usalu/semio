#!/bin/sh
# usage: lint-dir.sh <run-id> <nx|bazel> — official repo-lib:lint-dependency-direction verdict
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
id="$1"; tool="$2"; out="$C/🗑️generated/knot-f"; mkdir -p "$out/lint-art-$id"
s=$(date +%s)
SEMIO_TEST_ARTIFACT_DIR="/Users/ueli/Documents/semio/$out/lint-art-$id" SEMIO_LANGUAGE=en SEMIO_BUILD_GRAPH_POLICY=current nice -n 10 bun repo --tool="$tool" run @semio-tech/repo-lib:lint-dependency-direction > "$out/lint-$id.log" 2>&1
rc=$?
echo "lint-dependency-direction $id $tool rc=$rc $(( $(date +%s)-s ))s" >> "$out/verdicts.txt"
