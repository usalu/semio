#!/bin/bash
# usage: normts3-njs-chunk.sh <suite-list> <out>; runs each suite with its own bun test through the tool registry.
cd /home/deck/semio || exit 1
: > "$2"
while read -r f; do
  repo tools exec bun -- test "$f" > "$2.one" 2>&1
  echo "rc=$? $(grep -E '^ *[0-9]+ (pass|fail)$' "$2.one" | tr -s ' \n' ' ') $f" >> "$2"
  grep -q "fail$" "$2.one" && grep -E "^\(fail\)" "$2.one" >> "$2.fails"
done < "$1"
echo DONE >> "$2"
