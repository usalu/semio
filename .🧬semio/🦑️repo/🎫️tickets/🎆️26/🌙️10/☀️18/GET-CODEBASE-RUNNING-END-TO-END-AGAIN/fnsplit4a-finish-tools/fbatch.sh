#!/bin/zsh
# usage: fbatch.sh <from> <to> : fgo apply for lines <from>..<to> of cands.txt (one process at a time), summary per line
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
TD=$C/fnsplit4a-finish-tools
sed -n "$1,$2 p" $TD/cands.txt | while IFS= read -r row; do
  nm=$(echo "$row" | cut -d'|' -f2)
  echo "##### $nm"
  perl -e 'alarm 420; exec @ARGV' -- nice -n 10 $TD/fgo.sh "$row" 150 apply 2>&1 | cut -c1-260 | sed -E "s/produces \[.*\]/produces [..]/"
done
