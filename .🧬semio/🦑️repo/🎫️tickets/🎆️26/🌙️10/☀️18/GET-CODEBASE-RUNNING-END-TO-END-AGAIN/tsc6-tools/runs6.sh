#!/bin/zsh
setopt NO_BG_NICE
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cfg="$1"; tag="$2"
out="$C/🗑️generated/tsc6-logs/$tag.txt"
rm -f "$C/🗑️generated/tsc6-logs/$tag.done"
start=$(date +%s)
ovl="$C/🗑️generated/tsc6-info/$tag.tsconfig.json"
python3 "$C/tsc6-tools/mkoverlay.py" "$cfg" "$ovl" > /dev/null || exit 3
NODE_OPTIONS=--max-old-space-size=12288 perl -e 'alarm 3000; exec @ARGV' -- node node_modules/typescript/bin/tsc -p "$ovl" --noEmit --incremental --tsBuildInfoFile "$C/🗑️generated/tsc6-info/$tag.tsbuildinfo" --pretty false --extendedDiagnostics > "$out" 2>&1
rc=$?
end=$(date +%s)
errs=$(/usr/bin/grep -c -E "error TS[0-9]+" "$out")
files=$(/usr/bin/grep -E "^Files:" "$out" | awk '{print $2}')
echo "$tag rc=$rc errors=$errs files=$files secs=$((end-start))" > "$C/🗑️generated/tsc6-logs/$tag.done"
