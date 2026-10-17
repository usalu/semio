#!/bin/zsh
# usage: step.sh <tag> <cmd...>  (niced, ulimit, log + exit code in the monolith-split generated folder)
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
tag="$1"; shift
ulimit -n 65536
cd /Users/ueli/Documents/semio
nice -n 10 "$@" > "$F/🗑️generated/fb-$tag.log" 2>&1
echo $? > "$F/🗑️generated/fb-$tag.exit"
