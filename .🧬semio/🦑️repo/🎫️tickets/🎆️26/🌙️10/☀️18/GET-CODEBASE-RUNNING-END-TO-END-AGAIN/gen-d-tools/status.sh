#!/bin/sh
# gen-d status: sh status.sh <proof tag> ; prints the chain, the tail of the newest run log and the closure test log
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; G="$C/🗑️generated/gen-d"
date; sysctl -n vm.swapusage
grep -v "^$" "$G/$1/chain.txt" 2>/dev/null | grep -E "^run|^edited|^restored|^peer|failed" | cut -c1-200
for d in A A2 A3 B C; do [ -f "$G/$1/$d/run.log" ] && { echo "--- $d"; tail -c 230 "$G/$1/$d/run.log" | tr '\n' ' ' | cut -c1-230; echo; }; done
echo "--- closure tests"; tail -c 230 "$G/closure-tests.log" | tr '\n' ' ' | cut -c1-230; echo
