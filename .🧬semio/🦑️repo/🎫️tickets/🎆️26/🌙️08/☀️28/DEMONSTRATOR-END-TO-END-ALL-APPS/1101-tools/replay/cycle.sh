#!/bin/bash
# usage: cycle.sh <n>  — runs gate gaterp-<n>, waits, autofixes, lists remaining errors
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; "$C/1101-tools/gate.sh" "gaterp-$N" >/dev/null 2>&1
L="$C/🗑️generated/1101/gaterp-$N.txt"
echo "rc=$(cat "$C/🗑️generated/1101/gaterp-$N.rc")"
/usr/bin/grep 'could not compile' "$L" | sort -u | cut -c1-200
python3 "$C/1101-tools/replay/loop.py" "$PWD/$L"; python3 "$C/1101-tools/replay/arms.py" "$PWD/$L"; python3 "$C/1101-tools/replay/rstep.py" "$PWD/$L"; python3 "$C/1101-tools/replay/sigwrap.py" "$PWD/$L"; python3 "$C/1101-tools/replay/autowrap.py" "$PWD/$L"
python3 - "$L" <<'PY' | head -${2:-40}
import re,sys,os
s=open(sys.argv[1],errors='replace').read()
for m in re.finditer(r'error(?:\[E\d+\])?: ([^\n]*)\n\s*--> ([^\n]*?):(\d+):(\d+)\n',s):
    if 'could not' in m.group(1):continue
    f=os.path.normpath(m.group(2));l=int(m.group(3));c=int(m.group(4))
    try:t=open(f).read().split('\n')[l-1][c-1:c+60]
    except Exception:t=''
    print(f.split('semio/')[-1][-70:],l,c,m.group(1)[:70],'|',t)
PY
