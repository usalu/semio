#!/bin/bash
# usage: wait.sh <name>   (bounded <590s foreground wait)
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated/1029"
for i in $(seq 1 55); do [ -f "$C/testmig-$1.rc" ] && break; sleep 10; done
ls "$C/testmig-$1.rc" 2>/dev/null; tail -3 "$C/testmig-$1.txt" | cut -c1-300
