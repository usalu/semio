#!/bin/zsh
# usage: gate.sh <tag> <artifact-facade-dir>   (base snapshot at /Users/ueli/.cache/mwu/pre/<tag>)
cd /Users/ueli/Documents/semio || exit 99
ulimit -n 65536
FD=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
rm -rf /Users/ueli/.cache/mwu/sb-$1 /Users/ueli/.cache/mwu/sb-$1-combined /Users/ueli/.cache/mwu/sb-$1-base
nice -n 10 bun ./📜️script.ts new crates "$2" --check-applied --base=dir:/Users/ueli/.cache/mwu/pre/$1 --samples=60 --sandbox=/Users/ueli/.cache/mwu/sb-$1 > "$FD/🗑️generated/mwu-gate-$1.log" 2>&1
echo $? > "$FD/🗑️generated/mwu-gate-$1.exit"
rm -rf /Users/ueli/.cache/mwu/sb-$1 /Users/ueli/.cache/mwu/sb-$1-combined /Users/ueli/.cache/mwu/sb-$1-base
