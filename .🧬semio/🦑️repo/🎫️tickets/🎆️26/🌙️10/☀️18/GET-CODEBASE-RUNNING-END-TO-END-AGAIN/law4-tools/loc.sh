#!/bin/sh
# usage: loc.sh -> prints failing lines of last log
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
/usr/bin/grep -n -E "error|Expected|Received|✗|\(fail\)|ENOENT|^\s+[0-9]+ (pass|fail)" "$C/🗑️generated/law4-out/last.log" | head -${N:-40}
