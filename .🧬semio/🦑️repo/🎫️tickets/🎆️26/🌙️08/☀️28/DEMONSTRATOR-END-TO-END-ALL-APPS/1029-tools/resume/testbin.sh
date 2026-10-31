#!/bin/bash
# usage: testbin.sh <N> <package-dir relative to repo root> <test target> [libtest args, default --test-threads=1]   (native integration test target)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
rm -f "$C/🗑️generated/1029/resume-tb$1.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$2/Cargo.toml" --no-fail-fast --test "$3" -- ${4:---test-threads=1} > "$C/🗑️generated/1029/resume-tb$1.txt" 2>&1
echo $? > "$C/🗑️generated/1029/resume-tb$1.rc"
