#!/bin/sh
# usage: resume-nested-after-base.sh <attempts-file> <pid>...   (waits for the next "base attempt" line, then SIGCONTs the paused nested drivers)
f=$1; shift; n=$(grep -c "base attempt" "$f")
while [ "$(grep -c "base attempt" "$f")" -le "$n" ]; do sleep 30; done
for p in "$@"; do kill -CONT "$p"; done
echo "$(date +%H:%M:%S) resumed nested drivers $*" >> "$f"
