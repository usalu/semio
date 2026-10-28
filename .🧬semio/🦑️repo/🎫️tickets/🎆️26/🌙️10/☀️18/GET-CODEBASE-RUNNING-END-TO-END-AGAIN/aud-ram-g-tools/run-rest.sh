#!/bin/sh
# 📡 Finish the census: wait until the first monitor block ends, then tick every 290 s up to index 23.
T="$1"; M="$2"; S="$T/🗑️generated/aud-ram-g/samples.jsonl"
while kill -0 "$M" 2>/dev/null; do sleep 15; done
i=$(wc -l < "$S" | tr -d ' ')
while [ "$i" -le 23 ]; do
  sleep 290
  /usr/bin/python3 -I "$T/aud-ram-g-tools/tick.py" "$T/🗑️generated/aud-ram-g" "$i" 24 2>&1
  i=$((i+1))
done
echo "CENSUS DONE"
