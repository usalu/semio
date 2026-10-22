#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated"
N="$1"
S=$(date +%s)
nice -n 10 bun repo show targets --json > "$G/graph-proof2-$N.json" 2> "$G/graph-proof2-$N.err"
RC=$?
E=$(date +%s)
echo "name=$N rc=$RC start=$S end=$E wall=$((E-S))s" > "$G/graph-proof2-$N.time"
exit $RC
