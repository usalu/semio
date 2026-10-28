#!/bin/sh
cd /Users/ueli/Documents/semio
. "$(dirname "$0")/env.sh"
for sel in "@semio-tech/print-rs test" "@teaching/architecture-course build" "@semio-tech/repo-dashboard-rs build"; do
 for mode in before after; do
  if [ "$mode" = before ]; then X="EAGER=1 SLIM=0"; else X="EAGER=0 SLIM=1"; fi
  echo "== $sel $mode"
  env $X HOLD_ALL=1 STALE_OK=1 nice -n 10 /Users/ueli/.bun/bin/bun "$T/noop-closure.mjs" "$O/gen542" $sel 2>&1 | /usr/bin/grep -E '^\{' | python3 -c "
import sys,json
d=json.loads(sys.stdin.read());m={x['name']:x for x in d['marks']}
print('cpuMs',d['cpuMs'],'wallMs',d['wallMs'],'peakRssMB',d['peakRssMB'],'closure',m.get('closure(slice)',{}).get('projects'),'checkedRows',m.get('refreshClosure',{}).get('checked'))"
 done
done
