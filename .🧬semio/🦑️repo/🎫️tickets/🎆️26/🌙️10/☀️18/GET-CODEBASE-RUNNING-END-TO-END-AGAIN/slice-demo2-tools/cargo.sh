#!/bin/sh
# usage: cargo.sh <name> <cargo args...> -> log in 🗑️generated/slice-demo2/<name>.raw
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
n=$1; shift
SEMIO_LANGUAGE=en nice -n 10 bun repo cargo --store=shared --configuration=prototype -- "$@" > "$C/🗑️generated/slice-demo2/$n.raw" 2>&1
echo "rc=$?" >> "$C/🗑️generated/slice-demo2/$n.raw"
