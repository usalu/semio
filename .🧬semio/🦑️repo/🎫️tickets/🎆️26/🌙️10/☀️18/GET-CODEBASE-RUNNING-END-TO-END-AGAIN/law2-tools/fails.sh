#!/bin/zsh
# usage: fails.sh  -> prints failing tests (names) and error head from last.log
L="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/law2-out/last.log"
command grep -n "^(fail)\|^error:\|Expected\|Received\|^\s*at .*🟦️.ts:[0-9]" "$L" | head -${1:-60}
