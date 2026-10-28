#!/bin/sh
# usage: run.sh <build-relative file | /__abs__/path> <log> [name-pattern] [birth] -> genuine-birth bun test of one file; own pid in <log>.pid; nice 10
cd /Users/ueli/Documents/semio || exit 1
B=$(ls /Users/ueli/Documents/semio/.cache/b/t/m/bun/*/bun | head -1)
G="${GENUINE_SCRIPT:-🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/📜️script.ts}"
M="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
mkdir -p /Users/ueli/Documents/semio/temp/ng-birth/tmp
unset SEMIO_SCRIPT_STALL_MS
case "$1" in /__abs__*) F="${1#/__abs__}";; *) F="$M/$1";; esac
echo $$ > "$2.pid"
if [ -n "$3" ]; then
  TMPDIR=/Users/ueli/Documents/semio/temp/ng-birth/tmp SEMIO_WORKSPACE_ROOT=/Users/ueli/Documents/semio exec nice -n 10 "$B" "$G" "--file=$F" "--birth=${4:-setup-sdk}" "--pattern=$3" > "$2" 2>&1
else
  TMPDIR=/Users/ueli/Documents/semio/temp/ng-birth/tmp SEMIO_WORKSPACE_ROOT=/Users/ueli/Documents/semio exec nice -n 10 "$B" "$G" "--file=$F" "--birth=${4:-setup-sdk}" > "$2" 2>&1
fi
