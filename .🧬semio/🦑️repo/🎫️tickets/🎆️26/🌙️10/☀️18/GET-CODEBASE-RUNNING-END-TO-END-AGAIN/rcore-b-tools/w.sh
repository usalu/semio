#!/bin/sh
# usage: w.sh NAME : wait (bounded ~9 min) for the run NAME's wrapper; prints summary
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
N=$1; P=$2; O=$C/🗑️generated/rcore-b/$N.out
i=0; while [ $i -lt 54 ]; do kill -0 $P 2>/dev/null || break; sleep 10; i=$((i+1)); done
kill -0 $P 2>/dev/null && echo "RUNNING $P: $(tail -n 1 $O | cut -c1-160)" || echo "DONE $P"
/usr/bin/grep -E "^test .*(FAILED|failed)|test result|^error(\[|:)|panicked|Running " $O | cut -c1-300 | tail -n ${3:-25}
