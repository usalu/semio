#!/bin/bash
# usage: then-sweep.sh WAIT_PID MODE LIST RESULTS : waits for WAIT_PID, removes the stop flag, runs the sweep
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
while kill -0 "$1" 2>/dev/null; do sleep 20; done
rm -f "$C/🗑️generated/plug-al-b/stop"
exec bash "$C/plug-al-b-tools/sweep.sh" "$2" "$3" "$4"
