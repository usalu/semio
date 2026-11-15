#!/bin/bash
# usage: qa3-all.sh list-file -> runs qa3-run.sh for each asset with front azimuth 90
T="$(cd "$(dirname "$0")" && pwd)"; cd "$T"
while read a; do
  slug=${a#*/}; cat=${a%/*}
  FRONTAZ=90 ./qa3-run.sh $a $(python3 -c "
import json,sys
v=json.load(open('volumes/$a.json'))['volumes']
print(' '.join(k for k,x in v.items() if x.get('status')=='authored'))") >/dev/null 2>&1
  echo "done $a $(date +%H:%M:%S)" >> 🗑️generated/qa3/progress.txt
done < "$1"
