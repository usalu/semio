#!/bin/zsh
setopt NO_BG_NICE
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cfg="$1"; tag="$2"; overlay="${3:--}"
out="$C/🗑️generated/tsc-stage-logs/$tag.txt"
rm -f "$C/🗑️generated/tsc-stage-logs/$tag.done"
python3 "$C/tsc-stage-tools/stage.py" export "$C/🗑️generated/tsc-stage-work/overlay-$tag.json" > /dev/null
start=$(date +%s)
NODE_OPTIONS=--max-old-space-size=12288 perl -e 'alarm 3000; exec @ARGV' -- node "$C/tsc-stage-tools/tscheck.mjs" "$cfg" "$C/🗑️generated/tsc-stage-work/overlay-$tag.json" > "$out" 2>&1
rc=$?
errs=$(/usr/bin/grep -c -E "error TS[0-9]+" "$out")
echo "$tag rc=$rc errors=$errs secs=$(( $(date +%s) - start ))" > "$C/🗑️generated/tsc-stage-logs/$tag.done"
