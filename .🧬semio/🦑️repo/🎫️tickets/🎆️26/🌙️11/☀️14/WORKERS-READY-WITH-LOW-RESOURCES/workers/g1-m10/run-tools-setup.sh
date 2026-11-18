#!/bin/zsh
W="$1"; cd /Users/ueli/Documents/semio || exit 1
s=$(date +%s); st=$(date -u +%FT%TZ)
/Users/ueli/.local/bin/repo tools setup all > "$W/round-29.tools-setup.log" 2>&1
rc=$?
e=$(date +%s)
printf '{"start":"%s","end":"%s","rc":%d,"wall_seconds":%d,"attempt":6}\n' "$st" "$(date -u +%FT%TZ)" $rc $((e-s)) > "$W/round-29.tools-setup.metrics.json"
