#!/bin/sh
cd /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript || exit 2
SEMIO_TEST_LEVEL=${LEVEL:-quick} nice -n 10 node node_modules/vitest/vitest.mjs run --config "../../🧪️tests/🎚️config/🟦️.ts" --configLoader native --testTimeout 300000 --hookTimeout 300000 --teardownTimeout 300000 "$@" > "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/course-rest-vitest.out" 2>&1
echo rc=$? >> "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/course-rest-vitest.out"
