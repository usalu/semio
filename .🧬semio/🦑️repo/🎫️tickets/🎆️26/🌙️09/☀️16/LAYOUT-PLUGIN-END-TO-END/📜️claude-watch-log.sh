#!/usr/bin/env bash
# 👀️ Stream actionable lines of the newest ticket log matching a prefix (ticket 26/09/16/LAYOUT-PLUGIN-END-TO-END).
f=$(ls -t "$(dirname "$0")/🗑️generated/claude/$1"*.txt | head -1)
tail -n0 -F "$f" | grep --line-buffered -E 'error|Error|failed|FAILED|exit=|Compiling semio-hub|Finished|waiting [0-9]+0s|ready|Ready|http://' | grep --line-buffered -v lease-sweep | cut -c1-300
