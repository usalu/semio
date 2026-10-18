#!/bin/zsh
# usage: bt.sh <repo-relative test file>...  (bun test with the scratch node_modules via NODE_PATH and the workspace path map)
cd /Users/ueli/Documents/semio
args=(); for a in "$@"; do args+=("./$a"); done
NODE_PATH=/Users/ueli/.cache/pai/nm/node_modules exec bun --tsconfig-override=/Users/ueli/.cache/pai/nm/tsconfig.override.json test "${args[@]}"
