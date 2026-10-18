#!/bin/zsh
# usage: run-tjc.sh old|new : runs toolJobCoverageSelfTests instrumented (every throwing `if` condition and every throw is recorded, throws do not abort) under bun test with the c6fin2 scratch node_modules; writes C/🗑️generated/fnsplit-tjc-<mode>.json
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
cd /Users/ueli/Documents/semio
mkdir -p "$C/🗑️generated"
export TJC_MODE="$1"
export TJC_REAL_DIR="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-coverage"
export TJC_OLD_FILE="$C/fnsplit-tools/orig/tjc.ts"
export TJC_OUT="$C/🗑️generated/fnsplit-tjc-$1.json"
SEMIO_TEST_ARTIFACT_DIR=/Users/ueli/.cache/c6fin2/artifacts NODE_PATH=/Users/ueli/.cache/c6fin2/nm/node_modules perl -e 'alarm 600; exec @ARGV' -- bun --tsconfig-override=/Users/ueli/.cache/c6fin2/nm/tsconfig.override.json test "$C/fnsplit-tools/tjc-run.test.ts" 2>&1 | tail -15
