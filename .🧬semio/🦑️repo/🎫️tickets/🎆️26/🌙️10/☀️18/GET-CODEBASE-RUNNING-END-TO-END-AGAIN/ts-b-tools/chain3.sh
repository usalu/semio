#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
sh "$C/ts-b-tools/many.sh" m-small typecheck @semio-tech/pets,@semio-tech/pets-react,@semio-tech/ui-react,@semio-tech/framework-renderer-react,@semio-tech/course-react
sh "$C/ts-b-tools/many.sh" m-os typecheck @semio-tech/framework-os,@semio-tech/repo-lib
sh "$C/ts-b-tools/many.sh" m-sdev typecheck @semio-tech/s-dev-composition
