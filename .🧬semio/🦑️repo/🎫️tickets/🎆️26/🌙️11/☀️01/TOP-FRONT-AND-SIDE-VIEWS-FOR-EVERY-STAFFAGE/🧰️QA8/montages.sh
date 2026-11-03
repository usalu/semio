#!/bin/bash
# usage: montages.sh  -- one zoomed montage per asset and scale (all poses and clip instants) into 🗑️generated/QA8/mont
export PYTHONIOENCODING=utf-8
T="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE"
G="$T/🗑️generated/QA8"; mkdir -p "$G/mont"
for e in "mallard-duck 0.46 0.6" "pig 0.76 1.2" "pigeon 0.3 0.4" "red-fox 0.63 1.2" "red-squirrel 0.3 0.25" "roe-deer 1.08 1.5" "sheep 0.82 1.3"; do
  set -- $e
  for d in "$G"/sheets/$1-*; do
    k=$(basename "$d")
    python "$T/🧰️QA8/zoom.py" "$G/sheets" "$k" "$G/mont/$k" 130 "" 7 $2 $3 2>&1 | grep -v -i warn
  done
done
echo MONTAGES-DONE
