#!/bin/sh
# usage: cargo.sh <run-id> <cargo test args...>   (writes C/🗑️generated/snap-d/<run-id>.raw, ends with rc=)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
id=$1; shift
export SEMIO_BUILD_TERMINAL=false
nice -n 10 bun repo cargo --store=shared --configuration=prototype -- test "$@" --message-format=short
echo "rc=$?"
