#!/bin/sh
# waits for the typecheck chain (pid $1, lstart $2 must still match) then runs the package-export boundary test
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
while [ "$(ps -o lstart= -p "$1" 2>/dev/null | sed 's/^ *//')" = "$2" ]; do sleep 20; done
sh "$C/ext-f-tools/run.sh" bzuitest repo --tool=bazel run @semio-tech/ui-react:test -- "🧰️framework/🔨️modules/🖱️ui/🧪️tests/📦️react-package-export/🟦️.ts"
