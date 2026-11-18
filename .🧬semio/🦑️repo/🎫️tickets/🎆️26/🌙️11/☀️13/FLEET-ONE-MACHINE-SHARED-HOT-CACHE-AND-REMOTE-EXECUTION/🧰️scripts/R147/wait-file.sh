#!/bin/zsh
f=$1
for i in $(seq 1 26); do
 grep -q "verdict\|Verdict\|VERDICT" $f 2>/dev/null && { echo done; exit 0; }
 [ -z "$(pgrep -f 'slice-lab/📜️script.ts run --memory=16384 --cores=8 --machines=2 --slice=repo-mcp --timeout=540')" ] && { echo ended; exit 0; }
 sleep 20
done
echo waiting
