#!/bin/sh
cd /Users/ueli/Documents/semio || exit 2
nice -n 10 bun repo cargo --store=shared --configuration=prototype -- test --manifest-path 🎓️teaching/Cargo.toml -p teaching-proctor -p teaching-proctor-base -p teaching-proctor-domain > ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/course-rest-proctor-test.out" 2>&1
echo rc=$? >> ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/course-rest-proctor-test.out"
