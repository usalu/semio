#!/bin/bash
# usage: run.sh <tag> <verb> <args...>  -- launches c.sh detached, waits for its rc (<=9 min per call)
cd /Users/ueli/Documents/semio || exit 1
S=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$S/🗑️generated/1101"
tag=$1
if [ "$2" != wait ]; then rm -f "$G/compute-$tag.rc"; nohup "$S/1101-tools/compute/c.sh" "$@" >/dev/null 2>&1 & fi
for i in $(seq 1 17); do [ -f "$G/compute-$tag.rc" ] && break; sleep 30; done
cat "$G/compute-$tag.rc" 2>/dev/null; /usr/bin/grep -c "^error" "$G/compute-$tag.txt"
