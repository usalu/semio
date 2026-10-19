#!/bin/zsh
# usage: t.sh <relative-to-library path> [lines]
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
f="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/$1"
[ -f "$f" ] || f="🧰️framework/🛍️products/🦑️repo/🔨️modules/$1"
[ -f "$f" ] || f="$1"
python3 $C/tst3a-tools/r.py "$f" ${2:-20} 2>&1 | tail -${3:-25}
