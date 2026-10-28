#!/bin/sh
# usage: direct.sh <abs test file> <log> [name-pattern] -> plain bun test from /tmp (no genuine birth, no checkout bunfig); artifact dir under temp/obsrows-art
B=$(ls /Users/ueli/Documents/semio/.cache/b/t/m/bun/*/bun | head -1)
mkdir -p /Users/ueli/Documents/semio/temp/obsrows-art/$$ ; cd /tmp || exit 1
echo $$ > "$2.pid"
if [ -n "$3" ]; then
SEMIO_WORKSPACE_ROOT=/Users/ueli/Documents/semio SEMIO_TEST_ARTIFACT_DIR=/Users/ueli/Documents/semio/temp/obsrows-art/$$ exec nice -n 10 "$B" test --timeout 120000 -t "$3" "$1" > "$2" 2>&1
else
SEMIO_WORKSPACE_ROOT=/Users/ueli/Documents/semio SEMIO_TEST_ARTIFACT_DIR=/Users/ueli/Documents/semio/temp/obsrows-art/$$ exec nice -n 10 "$B" test --timeout 120000 "$1" > "$2" 2>&1
fi
