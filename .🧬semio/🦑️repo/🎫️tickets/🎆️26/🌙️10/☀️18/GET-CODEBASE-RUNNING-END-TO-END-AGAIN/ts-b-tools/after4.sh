#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
while kill -0 31678 2>/dev/null; do sleep 15; done
for p in @semio-tech/ui-react @semio-tech/pets-react @semio-tech/framework-renderer-react; do mv "$C/🗑️generated/ts-b/$(echo "$p-typecheck" | tr '/@:' '___').rc" "$C/🗑️generated/ts-b/prev-$(echo "$p" | tr '/@:' '___').rc" 2>/dev/null; done
sh "$C/ts-b-tools/chain.sh" typecheck @semio-tech/ui-react @semio-tech/pets-react @semio-tech/framework-renderer-react
