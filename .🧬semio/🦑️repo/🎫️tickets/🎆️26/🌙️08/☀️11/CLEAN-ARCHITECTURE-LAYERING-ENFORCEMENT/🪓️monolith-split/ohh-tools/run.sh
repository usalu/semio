#!/bin/zsh
# usage: run.sh <family> <label> [cfgs]
cd "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
ulimit -n 65536
export POS_PREFIX=ohh
exec nice -n 10 python3 ohh-tools/pos/pos.py run ohh-tools/families/$1.json --cfgs ${3:-lib,test} --no-base --label $2
