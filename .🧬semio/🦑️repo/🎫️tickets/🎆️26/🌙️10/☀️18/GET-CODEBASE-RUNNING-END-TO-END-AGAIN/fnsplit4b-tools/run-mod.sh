#!/bin/zsh
# usage: run-mod.sh <tag> <realFile> <snapshot> old|new : imports a module-level describe test file against a recording vitest shim; writes C/🗑️generated/fnsplit4b-reg-<tag>-<mode>.json
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
cd /Users/ueli/Documents/semio
mkdir -p "$C/🗑️generated"
export REG_MODE="$4" REG_REAL_FILE="/Users/ueli/Documents/semio/$2" REG_OLD_FILE="$C/fnsplit4b-tools/orig/$3"
export REG_OUT="$C/🗑️generated/fnsplit4b-reg-$1-$4.json"
SEMIO_TEST_ARTIFACT_DIR=/Users/ueli/.cache/c6fin2/artifacts NODE_PATH=/Users/ueli/.cache/c6fin2/nm/node_modules perl -e 'alarm 300; exec @ARGV' -- bun --tsconfig-override=/Users/ueli/.cache/c6fin2/nm/tsconfig.override.json test "$C/fnsplit4b-tools/mod-trace.test.ts" 2>&1 | command grep -E "RESULT|error:|pass|fail" | head -8
