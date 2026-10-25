#!/bin/sh
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
sh $C/ts-c-tools/many.sh mt2 test @semio-tech/framework,@semio-tech/ui-react,@semio-tech/framework-renderer-react
sh $C/ts-c-tools/many.sh m3 typecheck @semio-tech/framework-os
sh $C/ts-c-tools/many.sh mt3 test @semio-tech/framework-os
