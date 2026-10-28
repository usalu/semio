#!/bin/sh
# usage: chain.sh <listfile: rel[<TAB>birth[<TAB>pattern]]> <tag> -> runs run.sh per line sequentially (own pid in <tag>.chain.pid), results appended to generated/obsrows-g/<tag>.tsv, logs generated/obsrows-g/<tag>/<n>.log
CA=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
G=$CA/🗑️generated/obsrows-g; mkdir -p $G/$2; echo $$ > $G/$2.chain.pid
n=0
while IFS="	" read -r f birth pattern; do
  [ -z "$f" ] && continue
  n=$((n+1)); log=$G/$2/$n.log; start=$(date +%s)
  sh $CA/obsrows-g-tools/run.sh "$f" "$log" "$pattern" "${birth:-setup-sdk}"; rc=$?
  p=$(/usr/bin/grep -c "^(pass)" $log); fl=$(/usr/bin/grep -c "^(fail)" $log); sm=$(/usr/bin/grep -E "^ *[0-9]+ (pass|fail)$" $log | tr -s ' \n' ' ')
  printf '%s\t%s\trc=%s\t%ss\t%s\tlog=%s\n' "$f" "${birth:-setup-sdk}" "$rc" "$(( $(date +%s) - start ))" "$sm" "$log" >> $G/$2.tsv
done < "$1"
echo DONE >> $G/$2.tsv
