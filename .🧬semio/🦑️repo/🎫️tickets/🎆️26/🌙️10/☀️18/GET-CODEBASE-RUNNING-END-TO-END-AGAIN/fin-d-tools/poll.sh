#!/bin/sh
# usage: poll.sh <lines-seen>  sleeps up to ~520 s until chain.tsv has more lines than <lines-seen>, prints the new lines
cd /Users/ueli/Documents/semio || exit 1
O=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/fin-d"
i=0
while [ $i -lt 26 ]; do
  [ "$(wc -l < "$O/chain.tsv")" -gt "$1" ] && break
  sleep 20; i=$((i+1))
done
tail -n +$(( $1 + 1 )) "$O/chain.tsv" | cut -c1-200
echo "lines=$(wc -l < "$O/chain.tsv") $(date +%H:%M:%S)"
