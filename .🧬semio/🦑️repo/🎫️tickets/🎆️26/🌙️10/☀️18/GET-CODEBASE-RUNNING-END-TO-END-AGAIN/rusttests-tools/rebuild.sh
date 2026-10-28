#!/bin/sh
# 🔁 [rusttests] rebuilds scratch roots: S0 = base bytes (live + unlanded stack), S1 = S0 + edit.py; optionally syncs S1 into a mirror (arg 1). usage: sh rebuild.sh [mirror-root]
set -e; T="$(cd "$(dirname "$0")" && pwd)"; C="$(dirname "$T")"; G="$C/🗑️generated/rt"; mkdir -p "$G"
python3 -I "$T/rebuild.py" "$G/S0" "$G/S1"
[ -n "$1" ] && python3 -I "$T/syncmirror.py" "$G/S0" "$G/S1" "$1"
exit 0
