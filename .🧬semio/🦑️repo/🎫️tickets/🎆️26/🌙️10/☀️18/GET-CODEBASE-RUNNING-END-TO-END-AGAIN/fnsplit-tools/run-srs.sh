#!/bin/zsh
# usage: run-srs.sh old|new : registers the settlement tests against a recording vitest and a proxy dependency bag (test names, nesting, order and hashed callback sources); writes C/🗑️generated/fnsplit-srs-<mode>.json
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
cd /Users/ueli/Documents/semio
mkdir -p "$C/🗑️generated"
export SRS_MODE="$1"
export SRS_REAL_DIR="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️shardclient-reserved-response-settlement"
export SRS_OLD_FILE="$C/fnsplit-tools/orig/srs.ts"
export SRS_OUT="$C/🗑️generated/fnsplit-srs-$1.json"
SEMIO_TEST_ARTIFACT_DIR=/Users/ueli/.cache/c6fin2/artifacts NODE_PATH=/Users/ueli/.cache/c6fin2/nm/node_modules perl -e 'alarm 600; exec @ARGV' -- bun --tsconfig-override=/Users/ueli/.cache/c6fin2/nm/tsconfig.override.json test "$C/fnsplit-tools/srs-run.test.ts" 2>&1 | tail -15
