#!/bin/zsh
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
n=$(printf '%03d' $1)
if [ "$2" != "wait" ]; then rm -f "$C/🗑️generated/cc-b$n.done"; nohup bun "$C/cc-tools/run.mjs" "$C" "$1" "${@:3}" > "$C/🗑️generated/cc-run$n.out" 2>&1 & disown; fi
for i in $(seq 1 57); do [ -f "$C/🗑️generated/cc-b$n.done" ] && break; sleep 10; done
if [ -f "$C/🗑️generated/cc-b$n.done" ]; then cat "$C/🗑️generated/cc-b$n.done"; cut -c1-330 "$C/🗑️generated/cc-b$n.errs" | head -${ERRN:-40}; else echo "running (call: rb.sh $1 wait)"; fi
