#!/bin/sh
# 🔎 Author 17: validate one appliance with the measured volume rules and print the issues only (messages cut to 420 characters).
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
$BUN r3-validate.ts "appliances/$1" --volumes $2 2>&1 | grep -v "^\[" | cut -c1-420 | tail -${3:-25}
