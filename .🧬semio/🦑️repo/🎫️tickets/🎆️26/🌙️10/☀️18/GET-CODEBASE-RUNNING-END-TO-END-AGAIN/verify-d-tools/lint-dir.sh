#!/bin/sh
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
s=$(date +%s)
SEMIO_TEST_ARTIFACT_DIR="$C/🗑️generated/verify-d/lint-art" SEMIO_LANGUAGE=en SEMIO_BUILD_GRAPH_POLICY=current nice -n 10 bun repo --tool=nx run @semio-tech/repo-lib:lint-dependency-direction > "$C/🗑️generated/verify-d/lint-direction.log" 2>&1
rc=$?
printf 'lint-dependency-direction\t%s\t%ss\tbun repo --tool=nx run @semio-tech/repo-lib:lint-dependency-direction\n' "$rc" "$(( $(date +%s)-s ))" >> "$C/🗑️generated/verify-d/verdicts.tsv"
