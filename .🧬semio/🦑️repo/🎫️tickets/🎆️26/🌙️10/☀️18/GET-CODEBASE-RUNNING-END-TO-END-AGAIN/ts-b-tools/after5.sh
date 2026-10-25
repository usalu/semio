#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
while kill -0 66236 2>/dev/null; do sleep 15; done
sh "$C/ts-b-tools/chain.sh" typecheck @semio-tech/repo-lib @semio-tech/s-dev-composition
sh "$C/ts-b-tools/chain.sh" test @semio-tech/framework @semio-tech/pets @semio-tech/pets-react @semio-tech/ui-react @semio-tech/framework-renderer-react @semio-tech/course-react @semio-tech/framework-os
