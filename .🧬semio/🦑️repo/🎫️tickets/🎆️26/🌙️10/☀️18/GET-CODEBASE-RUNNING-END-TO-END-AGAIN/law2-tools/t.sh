#!/bin/zsh
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio
python3 $C/law2-tools/src.py "$1" "$2" "${3:-2200}"
python3 $C/law2-tools/cover.py "$1" "$2" | cut -c1-230
