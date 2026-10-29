#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
for t in test-build-command-line test-cargo-retirement; do
  rm -f "$C/🗑️generated/1029/diskcap2-$t.rc"
  SEMIO_LANGUAGE=en repo run repo:$t > "$C/🗑️generated/1029/diskcap2-$t.txt" 2>&1
  echo $? > "$C/🗑️generated/1029/diskcap2-$t.rc"
done
