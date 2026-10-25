#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
id=$1; shift
export SEMIO_BUILD_TERMINAL=false
nice -n 10 bun repo cargo --store=shared --configuration=prototype -- test --message-format=short "$@" > "$C/🗑️generated/kern-d/$id.raw" 2>&1
echo "rc=$?" >> "$C/🗑️generated/kern-d/$id.raw"
