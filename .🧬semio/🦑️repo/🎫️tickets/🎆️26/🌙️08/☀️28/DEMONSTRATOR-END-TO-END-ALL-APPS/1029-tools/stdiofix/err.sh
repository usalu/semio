#!/bin/bash
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated/1029"
/usr/bin/grep -v '^\[script\]' "$C/stdiofix-$1.txt" | /usr/bin/grep -vE '^warning|^\[capacity\]' | /usr/bin/grep -E -A12 '^error' | cut -c1-260 | head -${2:-200}
