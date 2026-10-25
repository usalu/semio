#!/bin/zsh
setopt NO_BG_NICE
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cfg="$1"; tag="$2"
G="$C/🗑️generated/tsc-d"
out="$G/$tag.txt"
rm -f "$G/$tag.done"
start=$(date +%s)
nice -n 10 bun repo tools exec tsc -- -p "$cfg" --noEmit --incremental --tsBuildInfoFile "$G/$tag.tsbuildinfo" --pretty false --extendedDiagnostics > "$out" 2>&1
rc=$?
end=$(date +%s)
errs=$(/usr/bin/grep -c -E "error TS[0-9]+" "$out")
files=$(/usr/bin/grep -E "^Files:" "$out" | awk '{print $2}')
echo "$tag rc=$rc errors=$errs files=$files secs=$((end-start))" > "$G/$tag.done"
