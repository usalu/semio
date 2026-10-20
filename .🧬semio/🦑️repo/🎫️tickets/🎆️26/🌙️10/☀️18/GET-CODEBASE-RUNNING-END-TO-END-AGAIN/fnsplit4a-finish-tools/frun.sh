#!/bin/zsh
# usage: frun.sh <candsLine>... : fgo apply for the given cands.txt line numbers into one log (foreground, one at a time)
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
TD=$C/fnsplit4a-finish-tools
for n in "$@"; do
  row=$(sed -n "${n}p" $TD/cands.txt); nm=$(echo "$row" | cut -d'|' -f2)
  echo "##### $n $nm"
  perl -e 'alarm 450; exec @ARGV' -- nice -n 10 $TD/fgo.sh "$row" 150 apply 2>&1 | cut -c1-300 | sed -E "s/produces \[.*\]/produces [..]/"
done
