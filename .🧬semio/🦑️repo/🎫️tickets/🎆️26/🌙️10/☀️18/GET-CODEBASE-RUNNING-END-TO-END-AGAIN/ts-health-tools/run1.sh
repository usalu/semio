#!/bin/zsh
setopt NO_BG_NICE
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/ts-health-run"
cfg="$1"; tag="$2"
mkdir -p "$G/logs" "$G/info"
rm -f "$G/logs/$tag.done"
start=$(date +%s)
NODE_OPTIONS=--max-old-space-size=12288 perl -e 'alarm 3000; exec @ARGV' -- nice -n 10 node node_modules/typescript/bin/tsc -p "$cfg" --noEmit --incremental --tsBuildInfoFile "$G/info/$tag.tsbuildinfo" --pretty false --extendedDiagnostics > "$G/logs/$tag.txt" 2>&1
rc=$?
end=$(date +%s)
errs=$(/usr/bin/grep -c -E "error TS[0-9]+" "$G/logs/$tag.txt")
files=$(/usr/bin/grep -E "^Files:" "$G/logs/$tag.txt" | awk '{print $2}')
echo "$tag rc=$rc errors=$errs files=$files secs=$((end-start))" > "$G/logs/$tag.done"
