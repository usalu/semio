#!/bin/zsh
# usage: bt.sh <repo-relative test file>...  (bun test with the scratch node_modules via NODE_PATH and the workspace path map)
cd /Users/ueli/Documents/semio
NODE_PATH=/Users/ueli/.cache/c5c8/nm/node_modules exec bun --tsconfig-override=/Users/ueli/.cache/c5c8/nm/tsconfig.override.json test "$@"
