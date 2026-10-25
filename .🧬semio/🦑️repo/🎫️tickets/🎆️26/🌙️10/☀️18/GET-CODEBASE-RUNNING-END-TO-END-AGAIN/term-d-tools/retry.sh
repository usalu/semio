#!/bin/sh
# usage: retry.sh <tag> <max> <crate>...   crates.sh per crate, repeated (60 s apart) while the build store flakes with E0463
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/term-d"
tag=$1; max=$2; shift 2
for c in "$@"; do
  n=1
  while :; do
    rm -f "$O/$tag-$c.sum"
    sh "$C/term-d-tools/crates.sh" "$tag-$c-$n" "$c"
    if /usr/bin/grep -q 'rc=0' "$O/$tag-$c-$n.sum"; then cp "$O/$tag-$c-$n-$c.log" "$O/$tag-$c.log"; echo "$(date +%H:%M:%S) $c OK attempt=$n $(/usr/bin/grep -E '^test result' "$O/$tag-$c.log" | tr '\n' ' ' | cut -c1-200)" >> "$O/$tag.sum"; break; fi
    if [ $n -ge $max ] || ! /usr/bin/grep -q 'E0463' "$O/$tag-$c-$n-$c.log"; then cp "$O/$tag-$c-$n-$c.log" "$O/$tag-$c.log"; echo "$(date +%H:%M:%S) $c FAILED attempt=$n" >> "$O/$tag.sum"; break; fi
    n=$((n+1)); sleep 60
  done
done
echo done >> "$O/$tag.sum"
