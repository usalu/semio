#!/bin/sh
# waits up to ~9 min for a plug-mz-c probe log NEWER than the one given ($1, default newest) to finish; prints its error summary
cd /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN || exit 1
BASE=${1:-$(ls -t 🗑️generated/plug-mz-c/w[0-9]*.log | head -1)}
i=0; L=$BASE
while [ $i -lt 52 ]; do
  N=$(ls -t 🗑️generated/plug-mz-c/w[0-9]*.log | head -1)
  if [ "$N" != "$BASE" ]; then L=$N; /usr/bin/grep -aq "could not compile\|Finished\|error: script" "$L" && { sleep 20; break; }; fi
  sleep 10; i=$((i+1))
done
echo "$L"; date
python3 plug-mz-c-tools/errs2.py "$L" 2>&1 | tail -10 | cut -c1-200
