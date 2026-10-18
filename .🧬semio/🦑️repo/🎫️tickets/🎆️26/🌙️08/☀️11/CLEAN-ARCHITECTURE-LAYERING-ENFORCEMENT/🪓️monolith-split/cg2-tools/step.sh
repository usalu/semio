#!/bin/zsh
# usage: step.sh <tag> <cmd...>  runs niced from the library dir, logs to F/🗑️generated/cg2-<tag>.log, exit code to cg2-<tag>.exit
cd /Users/ueli/Documents/semio || exit 99
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
tag=$1; shift
ulimit -n 65536
export LC_ALL=en_US.UTF-8
rm -f "$F/🗑️generated/cg2-$tag.exit"
cd /Users/ueli/Documents/semio || exit 98
"$@" > "$F/🗑️generated/cg2-$tag.log" 2>&1
echo $? > "$F/🗑️generated/cg2-$tag.exit"
