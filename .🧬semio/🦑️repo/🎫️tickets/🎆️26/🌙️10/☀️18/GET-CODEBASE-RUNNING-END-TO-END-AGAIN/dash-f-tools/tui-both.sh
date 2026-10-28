#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
for l in de en; do echo "### $l"; nice -n 10 sh "$C/dash-f-tools/tui-language.sh" $l; echo "rc=$?"; done
