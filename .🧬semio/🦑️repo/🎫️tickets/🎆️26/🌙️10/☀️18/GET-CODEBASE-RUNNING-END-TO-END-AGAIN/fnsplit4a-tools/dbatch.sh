#!/bin/zsh
# usage: dbatch.sh <from> <to> : runs dgo.sh with apply for lines from..to (1-based) of dlist.txt
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
T=$C/🗑️generated/fnsplit4a-census
cd /Users/ueli/Documents/semio
sed -n "$1,$2p" $T/dlist.txt | while IFS='|' read -r f l; do
  echo "=== $f:$l" | sed 's/.*\(.\{90\}\)$/\1/'
  perl -e 'alarm 500; exec @ARGV' -- $T/dgo.sh "$f" "$l" 165 apply 2>&1 | cut -c1-260
done
