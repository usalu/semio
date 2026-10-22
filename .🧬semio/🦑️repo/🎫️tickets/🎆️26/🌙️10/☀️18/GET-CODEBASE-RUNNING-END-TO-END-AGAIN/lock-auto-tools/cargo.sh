#!/bin/sh
# usage: cargo.sh <name> <cargo args…>: repo cargo ingress (shared store, prototype), policy current, nice 10, raw log in 🗑️generated/lock-auto/<name>.raw
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
n=$1; shift
SEMIO_LANGUAGE=${SEMIO_LANGUAGE:-en} nice -n 10 bun repo cargo --store=shared --configuration=prototype -- "$@" > "$C/🗑️generated/lock-auto/$n.raw" 2>&1
echo "rc=$?" >> "$C/🗑️generated/lock-auto/$n.raw"
