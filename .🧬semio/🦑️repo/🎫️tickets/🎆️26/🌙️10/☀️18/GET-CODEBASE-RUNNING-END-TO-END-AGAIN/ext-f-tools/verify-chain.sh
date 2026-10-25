#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
sh "$C/ext-f-tools/run.sh" vlit ./📜️script.ts verify dependencies literal-external
sh "$C/ext-f-tools/run.sh" vfreeze ./📜️script.ts verify dependencies
sh "$C/ext-f-tools/run.sh" vlocks ./📜️script.ts verify dependencies locks
