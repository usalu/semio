#!/bin/zsh
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
for i in "$@"; do
 n=$(printf '%03d' $i)
 rm -f "$C/🗑️generated/cc-b$n.done"
 CC_NOWAIT=1 bun "$C/cc-tools/run.mjs" "$C" "$i" > "$C/🗑️generated/cc-run$n.out" 2>&1
 echo "$(date +%H:%M) $(cat $C/🗑️generated/cc-b$n.done 2>/dev/null | head -1) idx=$i" >> "$C/🗑️generated/cc2-seq.log"
done
echo "$(date +%H:%M) SEQ DONE $*" >> "$C/🗑️generated/cc2-seq.log"
