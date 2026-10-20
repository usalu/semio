#!/bin/zsh
# usage: pbatch.sh <from> <to> : pgo with apply for lines from..to of plist.txt
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
T=$C/🗑️generated/fnsplit4a-census
cd /Users/ueli/Documents/semio
sed -n "$1,$2p" $T/plist.txt | while IFS='|' read -r f n l ln; do
  echo "=== $n ($ln) $(echo $f | sed 's/.*\(.\{60\}\)$/\1/')"
  perl -e 'alarm 560; exec @ARGV' -- $T/pgo.sh "$f" "$n" 150 apply 2>&1 | cut -c1-330
done
