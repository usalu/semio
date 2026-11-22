#!/bin/sh
# usage: roster-when-space.sh <minimum free GiB>; starts the single-slot roster once the host has that much free space
set -eu
T="$(cd "$(dirname "$0")" && pwd)"; need=$(( $1 * 1048576 ))
until [ "$(df -k / | awk 'NR==2{print $4}')" -ge "$need" ]; do sleep 30; done
python3 -c "import json,sys;p=sys.argv[1];s=json.load(open(p));s['active']=[];json.dump(s,open(p,'w'),indent=2)" "$T/🗑️generated/roster-pairs-state.json"
echo "[roster start gated $1 GiB] $(date)" >> "$T/🗑️generated/0751-expanded-roster.log"
ROSTER_SLOTS=1 exec python3 -B "$T/roster-pairs.py" >> "$T/🗑️generated/0751-expanded-roster.log" 2>&1
