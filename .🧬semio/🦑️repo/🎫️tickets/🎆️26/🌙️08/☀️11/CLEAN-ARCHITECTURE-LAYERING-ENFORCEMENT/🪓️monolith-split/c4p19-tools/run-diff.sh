#!/bin/zsh
# usage: run-diff.sh <label> [cfgs]   rustc resolution differential (original restored in a sandbox) for the eight groups; C4P19_LIVE=1 compares the installed tree
T=${0:A:h}
F=${T:h}
cd $T || exit 2
for g in iso16757 en1994 din4108 en1996 en1995 en1999 en1998 procedural; do
  python3 oracle.py $g ${2:-lib,test,p2} >> "$F/🗑️generated/c4p19-diff-$1.log" 2>&1
done
echo "done" >> "$F/🗑️generated/c4p19-diff-$1.exit"
