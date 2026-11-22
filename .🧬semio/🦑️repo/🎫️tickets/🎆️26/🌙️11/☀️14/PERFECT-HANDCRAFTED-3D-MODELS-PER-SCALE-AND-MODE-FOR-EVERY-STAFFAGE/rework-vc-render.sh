#!/bin/bash
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A=appliances/vacuum-cleaner
while IFS= read -r args; do [ -z "$args" ] && continue; $BUN r3-sheet.ts $A $args 2>&1 | tail -1; done
