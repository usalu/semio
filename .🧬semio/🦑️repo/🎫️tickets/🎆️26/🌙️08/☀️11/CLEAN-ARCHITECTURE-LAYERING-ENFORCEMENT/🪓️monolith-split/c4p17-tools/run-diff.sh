#!/bin/zsh
# usage: run-diff.sh <label> [cfgs] [groups...]   rustc resolution differential (sandbox with the original restored versus the staged or live split)
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
cd "$F/c4p17-tools" || exit 2
label=$1; cfgs=${2:-lib,test,p2}; shift 2
groups=("$@"); [ ${#groups} -eq 0 ] && groups=(value preview gen2d gen2dunit wfcbitmap wfc3d equation)
: > "$F/🗑️generated/c4p17-diff-$label.log"
for g in $groups; do
  python3 oracle.py $g $cfgs >> "$F/🗑️generated/c4p17-diff-$label.log" 2>&1
done
echo "label=$label done" >> "$F/🗑️generated/c4p17-diff.exit"
