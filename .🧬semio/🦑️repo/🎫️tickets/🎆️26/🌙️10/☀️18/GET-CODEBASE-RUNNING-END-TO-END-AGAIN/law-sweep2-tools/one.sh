#!/bin/zsh
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/law-sweep2"
cwd="$1"; shift
export LAW_GUARD_LOG="$PWD/$G/guard-one.log"; : > "$LAW_GUARD_LOG"
perl -e 'alarm 300; exec @ARGV' -- bun "$C/law-sweep2-tools/run-law.ts" "$cwd" "$@" 2>&1 | /usr/bin/grep -v '^\[script\]' | cut -c1-400 | sed 's#/Users/ueli/Documents/semio/##g' | head -${LINES_MAX:-25}
echo "rc=${pipestatus[1]} guard=$(cat $LAW_GUARD_LOG | cut -c1-100)"
