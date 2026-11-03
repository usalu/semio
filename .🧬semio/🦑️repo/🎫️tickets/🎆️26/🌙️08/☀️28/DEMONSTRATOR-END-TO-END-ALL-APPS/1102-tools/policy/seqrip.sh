#!/bin/bash
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1102"
date +%s > "$G/ripple.epoch"
"$C/1102-tools/policy/runchk.sh" "$1" native "$G/polRip.args"
