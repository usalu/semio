#!/bin/sh
# usage: lane.sh <name> <project:target>   (real Bazel lane, one at a time)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
S="$C/🗑️generated/repo-tests-b/lanes"; mkdir -p "$S"
nice -n 10 bun repo --tool=bazel run "$2" > "$S/$1.out" 2> "$S/$1.err"
echo "rc=$?" >> "$S/$1.err"
