#!/bin/sh
# waits for the cargo run (pid $1, lstart $2 must still match), then the four official Bazel proofs one at a time
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
while [ "$(ps -o lstart= -p "$1" 2>/dev/null | sed 's/^ *//')" = "$2" ]; do sleep 20; done
sh "$C/ext-f-tools/run.sh" bzuitc repo --tool=bazel run @semio-tech/ui-react:typecheck
sh "$C/ext-f-tools/run.sh" bzuitest repo --tool=bazel run @semio-tech/ui-react:test -- "🧰️framework/🔨️modules/🖱️ui/🧪️tests/📦️react-package-export/🟦️.ts"
sh "$C/ext-f-tools/run.sh" bzcomp repo --tool=bazel run @semio-tech/framework-compiler-rs:test-native
sh "$C/ext-f-tools/run.sh" bzpix repo --tool=bazel run semio-framework-pixels:test
