#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
mkdir -p "$C/🗑️generated/boot-rebind2/art"
exec nice -n 10 perl -e 'alarm 600; exec @ARGV' -- bun "$C/boot-rebind2-tools/run-test.ts" "./$1" "$2" "$PWD/$C/🗑️generated/boot-rebind2/art"
