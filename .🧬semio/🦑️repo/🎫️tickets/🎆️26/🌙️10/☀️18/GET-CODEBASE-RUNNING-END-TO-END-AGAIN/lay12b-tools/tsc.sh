#!/bin/zsh
cd /Users/ueli/Documents/semio
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/lay12b-run"
node_modules/.bin/tsc --noEmit -p "$G/tsconfig.json" > "$G/tsc-$1.txt" 2>&1
grep "error TS" "$G/tsc-$1.txt" | sed -E 's/\([0-9]+,[0-9]+\)//' | sort > "$G/tsc-$1-sorted.txt"
echo "errors: $(grep -c 'error TS' $G/tsc-$1.txt)"
if [ -f "$G/tsc-before-sorted.txt" ] && [ "$1" != "before" ]; then echo "--- new errors vs baseline:"; comm -13 "$G/tsc-before-sorted.txt" "$G/tsc-$1-sorted.txt" | cut -c1-330 | head -${2:-80}; fi
