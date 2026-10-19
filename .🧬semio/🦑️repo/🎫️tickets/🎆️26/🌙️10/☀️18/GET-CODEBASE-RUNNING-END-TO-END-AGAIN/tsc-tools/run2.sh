#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cfg="$1"; tag="$2"
out="$C/🗑️generated/tsc2-logs/$tag.txt"
start=$(date +%s)
NODE_OPTIONS=--max-old-space-size=12288 node node_modules/typescript/bin/tsc --noEmit -p "$cfg" --pretty false --extendedDiagnostics > "$out" 2>&1
rc=$?
end=$(date +%s)
errs=$(/usr/bin/grep -c -E "error TS[0-9]+" "$out")
files=$(/usr/bin/grep -E "^Files:" "$out" | awk '{print $2}')
echo "$tag rc=$rc errors=$errs files=$files secs=$((end-start))"
