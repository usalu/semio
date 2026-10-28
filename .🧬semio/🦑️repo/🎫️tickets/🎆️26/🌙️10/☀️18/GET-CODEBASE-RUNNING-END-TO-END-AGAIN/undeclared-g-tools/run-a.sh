#!/bin/sh
# run-a.sh <tag> <id> <list>: one tier A target through the pipeline (closure rows of the three roots)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; G="$C/🗑️generated/undeclared-g"
nice bun "$C/undeclared-g-tools/target-run.mjs" "$1" "$G/closure-roots.jsonl" "$2" "$G/lists-a/$3.files" "${@:4}"
