#!/bin/zsh
# usage: sweep.sh <manifest-dir-rel>...  -> static resolve-level sweep (plugfix sweep.py) + classify of only the given crates
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
rm -rf /Users/ueli/.cache/plugfix/raw
python3 "$C/plugfix-tools/sweep.py" "$@" > /dev/null 2>&1
python3 "$C/plugfix-tools/classify.py" 2>&1 | command grep -v "^  NOISE" | tail -${TAILN:-40}
