#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
unset SEMIO_SCRIPT_STALL_MS
nice -n 10 bun repo --tool=semio run @semio-tech/repo-lib:probe-nx-owner-arguments --exclude-dependencies --skip-cache -- long -E 'test(=native_name)' '' 'raw "quote"' > "$C/🗑️generated/dbgv-d/real/probe-args.log" 2>&1
echo $? > "$C/🗑️generated/dbgv-d/real/probe-args.rc"
