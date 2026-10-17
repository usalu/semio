#!/bin/zsh
# usage: live-checks.sh   static checks of the installed split against the staged plan (no cargo)
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
cd "$F/c4p18-tools" || exit 2
tags=(envelope gisedit owned gisroot vcsedit animation video scene)
for t in $tags; do python3 c4p18run.py verify $t; python3 c4p18run.py lossless $t | /usr/bin/grep -E 'LOSSLESS|missing [1-9]|UNEXPLAINED'; python3 c4p18run.py paths $t; python3 c4p18run.py refs $t; done
python3 emojis.py $tags
LIVE=1 python3 c4p18fmt.py $tags
