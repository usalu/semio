#!/bin/sh
# Ticket input script (dl-c): runs one explicit bun test file under a finite Original process envelope, as the repo test harness does.
#   sh runt.sh <file-from-repo-root>
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
mkdir -p /private/tmp/claude-501/dl-c-art
wire=$(LAW1_MAX_MS=${LAW1_MAX_MS:-3600000} bun "$C/law1-tools/envwire.ts")
SEMIO_TEST_ARTIFACT_DIR=/private/tmp/claude-501/dl-c-art NO_COLOR=1 SEMIO_REPO_ROOT=/Users/ueli/Documents/semio SEMIO_SCRIPT_PROCESS_INVOCATION="$wire" SEMIO_BUILD_MODE=prototype exec nice -n 10 bun test --timeout 120000 "./$1"
