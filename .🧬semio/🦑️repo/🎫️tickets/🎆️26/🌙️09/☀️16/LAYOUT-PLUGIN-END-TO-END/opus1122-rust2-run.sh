#!/bin/bash
cd /home/deck/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END"
G="$T/🗑️generated/opus1122"
ORDER="model engine mutations io examples"
list=$(cat $G/rust2-crates.txt)
first=""; for k in $ORDER; do first="$first $(echo "$list" | grep -E "layout-$k\$")"; done
rest=$(echo "$list" | grep -vxF -f <(echo $first | tr ' ' '\n'))
for c in $first $rest; do
  a=$(df -BG --output=avail /home | tail -1 | tr -dc 0-9)
  if [ "$a" -lt 5 ]; then SEMIO_LANGUAGE=en repo cache cas gc >/dev/null 2>&1; a=$(df -BG --output=avail /home | tail -1 | tr -dc 0-9); fi
  if [ "$a" -lt 4 ]; then echo "PAUSE disk ${a}G before $c" >> $G/rust2-summary.txt; exit 2; fi
  repo cargo --store=shared --configuration=prototype -- test -p $c > $G/rust2-$c.log 2>&1
  echo "$c exit=$? $(grep -E '^test result' $G/rust2-$c.log | tr '\n' ' ')" >> $G/rust2-summary.txt
done
echo DONE >> $G/rust2-summary.txt
