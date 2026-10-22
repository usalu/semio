#!/bin/sh
cd /Users/ueli/Documents/semio || exit 2
nice -n 10 node_modules/.bin/tsc -p "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/tsconfig.json" --noEmit --incremental --tsBuildInfoFile ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/course-rest-tsc.tsbuildinfo" > ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/course-rest-tsc.out" 2>&1
echo rc=$? >> ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/course-rest-tsc.out"
