#!/bin/sh
# usage: fail.sh <file> [lines] -> run + excerpt of first failures
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
mkdir -p "$C/🗑️generated/repo-tests-c/one/logs"
sh "$C/repo-tests-c-tools/one.sh" "$1" > "$C/🗑️generated/repo-tests-c/one/logs/0001.log" 2>&1
/usr/bin/grep '^ *[0-9]* \(pass\|fail\)' "$C/🗑️generated/repo-tests-c/one/logs/0001.log" | tr -s ' \n' ' '; echo
python3 "$C/repo-tests-c-tools/ex.py" "$C/🗑️generated/repo-tests-c/one" 1 ${2:-14} | cut -c1-260
