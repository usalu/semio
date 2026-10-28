#!/bin/sh
# 📡 Census 2 driver: wait for the first two ticks, then one tick every 290 s up to index 23 (stdlib sampler of aud-ram-g).
T="$1"; S="$T/samples.jsonl"; K="$T/../../aud-ram-g-tools"
while [ "$(wc -l < "$S" | tr -d ' ')" -lt 2 ]; do sleep 5; done
i=2
while [ "$i" -le 23 ]; do
  sleep 290
  /usr/bin/python3 -I "$K/tick.py" "$T" "$i" 24 2>&1
  i=$((i+1))
done
echo "CENSUS DONE"
