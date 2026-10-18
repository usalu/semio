#!/bin/zsh
# usage: run-vitest.sh <test-dir> <json-out> [include-glob,...] : runs vitest with a narrow root so it never scans the repo
cd /Users/ueli/Documents/semio
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
export CLS_TEST_ROOT="$1"
[ -n "$3" ] && export CLS_TEST_INCLUDE="$3"
mkdir -p /private/tmp/claude-501/cls-artifacts
export SEMIO_TEST_ARTIFACT_DIR=/private/tmp/claude-501/cls-artifacts
export NODE_PATH=/Users/ueli/.cache/cls/nm
NODE_OPTIONS=--max-old-space-size=6144 perl -e 'alarm 900; exec @ARGV' -- node_modules/.bin/vitest run --config "$C/cls-tools/vitest.config.ts" --root "$1" --reporter=json --outputFile="$2" 2>&1 | tail -5
