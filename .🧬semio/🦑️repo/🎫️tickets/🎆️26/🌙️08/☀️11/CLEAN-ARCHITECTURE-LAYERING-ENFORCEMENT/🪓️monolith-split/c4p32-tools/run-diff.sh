#!/bin/zsh
# usage: run-diff.sh <label> [cfgs] [groups...]   rustc resolution differential (original restored in a sandbox) for the c4p32 groups; C4P32_LIVE=1 compares the installed tree
T=${0:A:h}
F=${T:h}
label=$1; cfgs=${2:-lib,test,p2}; shift 2
cd $T || exit 2
for g in ${@:-femeditor femsession bim}; do
  python3 oracle.py $g $cfgs >> "$F/🗑️generated/c4p32-diff-$label.log" 2>&1
done
echo "done" >> "$F/🗑️generated/c4p32-diff-$label.exit"
