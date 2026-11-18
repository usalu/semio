#!/bin/zsh
# usage: battery.sh <out file> <mode: staged|base> <list file of test paths relative to the root>
T=${0:h}; R=/Users/ueli/Documents/semio
: > "$1"
while IFS= read -r f; do
 [ -z "$f" ] && continue
 if [ "$2" = base ]; then out=$($T/run.sh "$f" base 2>&1); elif [ "$2" = live ]; then out=$($T/run.sh "$f" live 2>&1); else out=$($T/run.sh "$f" 2>&1); fi
 echo "$f :: $(echo "$out" | grep -E '^ *[0-9]+ (pass|fail)$' | tr '\n' ' ')" >> "$1"
done < "$3"
