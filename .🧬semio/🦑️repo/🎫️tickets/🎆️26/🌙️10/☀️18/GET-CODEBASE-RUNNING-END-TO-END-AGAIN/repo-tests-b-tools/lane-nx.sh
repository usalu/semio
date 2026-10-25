#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
S="$C/🗑️generated/repo-tests-b/lanes"; mkdir -p "$S"
nice -n 10 bun repo --tool=nx run "$2" > "$S/$1.out" 2> "$S/$1.err"
echo "rc=$?" >> "$S/$1.err"
