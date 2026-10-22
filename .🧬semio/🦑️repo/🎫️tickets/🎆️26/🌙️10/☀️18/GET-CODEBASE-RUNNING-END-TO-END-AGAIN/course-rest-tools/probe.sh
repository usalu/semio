#!/bin/sh
cd /Users/ueli/Documents/semio || exit 2
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated"
n=$1
nice -n 10 bun repo show projects --json > "$G/course-rest-probe$n.out" 2> "$G/course-rest-probe$n.err"
rc=$?
echo rc=$rc
grep -v '^\[script\]\|elapsedMs\|^\[capacity\|^[0-9]* |\|^$' "$G/course-rest-probe$n.err" | cut -c1-500 | head -${2:-6}
