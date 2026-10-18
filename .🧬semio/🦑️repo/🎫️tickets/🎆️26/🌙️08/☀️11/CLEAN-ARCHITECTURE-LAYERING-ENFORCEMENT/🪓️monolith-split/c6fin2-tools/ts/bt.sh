#!/bin/zsh
# usage: bt.sh <repo-relative test file>...  (bun test with the scratch node_modules via NODE_PATH and the workspace path map copied from the pai cache)
cd /Users/ueli/Documents/semio
args=(); for a in "$@"; do args+=("./$a"); done
SEMIO_TEST_ARTIFACT_DIR=/Users/ueli/.cache/c6fin2/artifacts NODE_PATH=/Users/ueli/.cache/c6fin2/nm/node_modules exec nice -n 10 perl -e 'alarm 600; exec @ARGV' -- bun --tsconfig-override=/Users/ueli/.cache/c6fin2/nm/tsconfig.override.json test "${args[@]}"
