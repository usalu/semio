#!/bin/sh
# Ticket input script: runs one bun repo command from the repo root at nice 10 and records its exit code.
#   run.sh <name> <bun repo args...>      log: C/🗑️generated/crel-d/<name>.log, exit code: <name>.rc
here=$(cd "$(dirname "$0")" && pwd)
ticket=$(cd "$here/.." && pwd)
root=$(cd "$ticket" && git rev-parse --show-toplevel)
generated="$ticket/$(printf '\360\237\227\221\357\270\217generated')/crel-d"
mkdir -p "$generated"
name=$1; shift
cd "$root" || exit 3
rm -f "$generated/$name.rc"
nice -n 10 bun repo "$@" >>"$generated/$name.log" 2>&1
code=$?
echo "$code" >"$generated/$name.rc"
exit "$code"
