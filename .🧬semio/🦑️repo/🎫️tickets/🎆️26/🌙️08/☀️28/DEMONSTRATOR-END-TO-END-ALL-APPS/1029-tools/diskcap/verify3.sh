#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
SEMIO_LANGUAGE=en repo --tool=bazel run workspace:deps-browsers > "$C/🗑️generated/1029/diskcap3-deps-browsers.txt" 2>&1; echo $? > "$C/🗑️generated/1029/diskcap3-deps-browsers.rc"
SEMIO_LANGUAGE=en repo run repo:test-storage-budget > "$C/🗑️generated/1029/diskcap3-storage-budget.txt" 2>&1; echo $? > "$C/🗑️generated/1029/diskcap3-storage-budget.rc"
