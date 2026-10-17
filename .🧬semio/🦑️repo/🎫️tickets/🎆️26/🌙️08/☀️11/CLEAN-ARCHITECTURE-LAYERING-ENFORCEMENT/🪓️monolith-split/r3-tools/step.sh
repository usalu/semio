#!/bin/zsh
# usage: step.sh <tag> <cmd...>  runs niced in repo root, logs to F/🗑️generated/r3-<tag>.log, exit code to r3-<tag>.exit
cd /Users/ueli/Documents/semio || exit 99
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
tag=$1; shift
ulimit -n 65536
rm -f "$F/🗑️generated/r3-$tag.exit"
nice -n 10 "$@" > "$F/🗑️generated/r3-$tag.log" 2>&1
echo $? > "$F/🗑️generated/r3-$tag.exit"
