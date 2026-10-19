#!/bin/zsh
# usage: run-reg.sh <tag> <realFile> <snapshot> <fnName> old|new [argc] [vitestArg] [sourceArg]: registers a test function against a recording vitest and proxy dependency bags; writes C/🗑️generated/fnsplit2-reg-<tag>-<mode>.json
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
cd /Users/ueli/Documents/semio
mkdir -p "$C/🗑️generated"
export REG_MODE="$5" REG_REAL_FILE="/Users/ueli/Documents/semio/$2" REG_OLD_FILE="$C/fnsplit2-tools/orig/$3" REG_FN="$4" REG_ARGC="${6:-3}" REG_VITEST_ARG="${7:-0}" REG_SOURCE_ARG="${8:-$(( ${6:-3} - 1 ))}"
export REG_OUT="$C/🗑️generated/fnsplit2-reg-$1-$5.json"
SEMIO_TEST_ARTIFACT_DIR=/Users/ueli/.cache/c6fin2/artifacts NODE_PATH=/Users/ueli/.cache/c6fin2/nm/node_modules perl -e 'alarm 300; exec @ARGV' -- bun --tsconfig-override=/Users/ueli/.cache/c6fin2/nm/tsconfig.override.json test "$C/fnsplit2-tools/reg-trace.test.ts" 2>&1 | tail -12
