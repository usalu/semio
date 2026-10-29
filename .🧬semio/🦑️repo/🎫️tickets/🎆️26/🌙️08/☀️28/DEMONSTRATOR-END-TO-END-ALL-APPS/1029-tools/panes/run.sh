#!/usr/bin/env bash
cd /Users/ueli/Documents/semio || exit 1
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated/1029"
SEMIO_LANGUAGE=en repo --tool=bazel run @semio-tech/mit-bestand-demonstrator:test > "$G/panes-test.txt" 2>&1; echo $? > "$G/panes-test.rc"
SEMIO_LANGUAGE=en repo --tool=bazel run @semio-tech/mit-bestand-demonstrator:test-runtime-catalog > "$G/panes-catalog.txt" 2>&1; echo $? > "$G/panes-catalog.rc"
