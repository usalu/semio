#!/bin/sh
# waits for the bazel chain (pid $1, lstart $2 must still match) then runs the ui-react typecheck
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
while [ "$(ps -o lstart= -p "$1" 2>/dev/null | sed 's/^ *//')" = "$2" ]; do sleep 20; done
sh "$C/ext-f-tools/run.sh" bzuitc repo --tool=bazel run @semio-tech/ui-react:typecheck
