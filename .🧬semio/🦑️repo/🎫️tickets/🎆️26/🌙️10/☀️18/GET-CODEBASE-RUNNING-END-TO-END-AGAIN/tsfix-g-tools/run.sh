#!/bin/zsh
setopt NO_BG_NICE
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
out="$C/🗑️generated/tsfix-g/${1:-graph}.txt"
nice -n 10 bun repo tools exec tsc -- -p "$C/tsfix-g-tools/tsconfig.${1:-graph}.json" --noEmit --incremental --tsBuildInfoFile "$C/🗑️generated/tsfix-g/${1:-graph}.tsbuildinfo" --pretty false --extendedDiagnostics > "$out" 2>&1
echo "rc=$? errors=$(/usr/bin/grep -c -E 'error TS[0-9]+' "$out") files=$(/usr/bin/grep -E '^Files:' "$out" | awk '{print $2}')"
