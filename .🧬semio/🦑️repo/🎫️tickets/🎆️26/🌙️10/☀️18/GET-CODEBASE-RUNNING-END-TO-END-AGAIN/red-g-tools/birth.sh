#!/bin/sh
# usage: birth.sh <root> <build-relative test file> <log> [birth]  genuine-birth bun test of one library file with <root> as the workspace (live root or a mirror)
root="$1"; rel="$2"; log="$3"; birth="${4:-setup-sdk}"
B=$(ls /Users/ueli/Documents/semio/.cache/b/t/m/bun/*/bun | head -1)
G="$root/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/📜️script.ts"
F="$root/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/$rel"
mkdir -p "$root/temp/ng-birth/tmp"; cd "$root" || exit 1
unset SEMIO_SCRIPT_STALL_MS
TMPDIR="$root/temp/ng-birth/tmp" SEMIO_WORKSPACE_ROOT="$root" exec nice -n 10 "$B" "$G" "--file=$F" "--birth=$birth" > "$log" 2>&1
