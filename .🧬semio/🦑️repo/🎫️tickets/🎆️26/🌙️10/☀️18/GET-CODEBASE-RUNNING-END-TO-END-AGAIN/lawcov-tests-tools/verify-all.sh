#!/bin/zsh
# 🧪 Runs every installed lawcov-tests file, one bun process at a time: verify-all.sh <list> <summary>
list=$1; summary=$2; : > $summary
while read f; do
  d=$(dirname "$f"); b=$(basename "$f")
  out=$(cd "$d" && nice -n 10 bun test ./"$b" 2>&1 | tail -6 | tr '\n' ' ')
  echo "$(basename $(dirname "$f")) | $out" >> $summary
done < $list
