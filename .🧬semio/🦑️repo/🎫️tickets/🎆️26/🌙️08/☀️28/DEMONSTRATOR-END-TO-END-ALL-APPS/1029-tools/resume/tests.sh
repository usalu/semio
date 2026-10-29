#!/bin/bash
# usage: tests.sh <N> <check|test> <crate-dir> [test filter]   (native --lib --tests of one companion test crate)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
rm -f "$C/🗑️generated/1029/resume-tests$1.rc"
if [ "$2" = check ]; then
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$3/Cargo.toml" --tests --keep-going > "$C/🗑️generated/1029/resume-tests$1.txt" 2>&1
else
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$3/Cargo.toml" --lib "$4" > "$C/🗑️generated/1029/resume-tests$1.txt" 2>&1
fi
echo $? > "$C/🗑️generated/1029/resume-tests$1.rc"
