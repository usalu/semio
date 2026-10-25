#!/bin/sh
# usage: chain.sh <target> <project...>
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
t=$1; shift
for p in "$@"; do sh "$C/ts-b-tools/one.sh" "$p" "$t"; done
