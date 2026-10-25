#!/bin/sh
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
sh $C/ts-c-tools/many.sh m2 typecheck @semio-tech/repo-lib,@semio-tech/framework-os
sh $C/ts-c-tools/many.sh m1 typecheck @semio-tech/s-dev-composition
sh $C/ts-c-tools/many.sh mt test @semio-tech/framework,@semio-tech/pets,@semio-tech/pets-react,@semio-tech/ui-react,@semio-tech/framework-renderer-react,@semio-tech/course-react,@semio-tech/framework-os
