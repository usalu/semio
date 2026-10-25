#!/bin/sh
# usage: census-env.sh <outdirname> [wall] [list] — one exec envelope (real route environment), every listed file in its own process under an authored envelope
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
unset SEMIO_SCRIPT_STALL_MS SEMIO_TEST_ARTIFACT_DIR
list="${3:-$C/🗑️generated/proc-d/census-list.txt}"
nice -n 10 bun repo --tool=semio exec --projects=@semio-tech/framework-process -- bun "/Users/ueli/Documents/semio/$C/proc-d-tools/census-run.ts" "/Users/ueli/Documents/semio/$list" "/Users/ueli/Documents/semio/$C/🗑️generated/proc-d/$1" "${2:-420}" > "$C/🗑️generated/proc-d/$1.out" 2>&1
echo $? > "$C/🗑️generated/proc-d/$1.rc"
