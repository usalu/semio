#!/bin/sh
# usage: t.sh <file> <name-pattern>  runs one bun test (direct), prints error lines
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; O="$C/🗑️generated/dtest-d"
export SEMIO_TEST_ARTIFACT_DIR="$PWD/$O/art"
nice -n 10 bun test "./$1" -t "$2" --timeout 300000 > "$O/t-one.log" 2>&1; echo rc=$?
/usr/bin/grep -v '^\[' "$O/t-one.log" | /usr/bin/grep -n 'rror\|Expected\|Received\|^ *at \|(fail)\|(pass)' | cut -c1-600 | head -${3:-30}
