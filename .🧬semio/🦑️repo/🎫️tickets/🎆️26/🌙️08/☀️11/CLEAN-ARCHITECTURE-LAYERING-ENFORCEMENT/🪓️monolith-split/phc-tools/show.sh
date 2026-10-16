#!/bin/zsh
# usage: show.sh <path-fragment-regex> [lines]
cd "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/phc-stage"
f=$(ls | /usr/bin/grep -E "$1" | head -1)
echo "### $f"
head -${2:-60} "$f" | cut -c1-${3:-220}
