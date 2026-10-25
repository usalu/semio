#!/bin/sh
# usage: jchain.sh <tag>  builds binary + journeys executable (graph-free, retried on E0463) and runs the 35 journeys directly
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/term-d"; t=$1
for n in 1 2 3 4; do sh "$C/term-d-tools/jbin.sh" "$t-b$n"; /usr/bin/grep -q "rc=0" "$O/$t-b$n.done" && break; sleep 60; done
for n in 1 2 3 4; do sh "$C/term-d-tools/jbuild.sh" "$t-j$n"; /usr/bin/grep -q "rc=0" "$O/$t-j$n.done" && break; sleep 60; done
bin=$(head -1 "$O/$t-b$n.exe" 2>/dev/null); 
b=$(ls -t "$O"/$t-b*.exe | head -1); j=$(ls -t "$O"/$t-j*.exe | head -1)
sh "$C/term-d-tools/jrun.sh" "$t-run" "$(head -1 "$b")" "$(head -1 "$j")"
echo done > "$O/$t.fin"
