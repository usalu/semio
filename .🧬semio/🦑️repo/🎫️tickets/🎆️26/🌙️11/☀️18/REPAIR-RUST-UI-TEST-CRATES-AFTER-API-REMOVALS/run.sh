#!/bin/zsh
# usage: run.sh <tag> <crates-tsv> ; runs each crate's tests, one log per crate, summary in 🗑️generated/<tag>-summary.tsv
F="/Users/ueli/Documents/semio/🧰️framework"; G="${0:A:h}/🗑️generated"; tag=$1; cd "$F"
while IFS=$'\t' read -r name m extra; do
  log="$G/$tag-$name.log"; s=$(date +%s)
  timeout 1800 repo cargo --store=isolated --configuration=prototype -- test --offline --no-fail-fast --manifest-path "$F/$m" ${=extra} > "$log" 2>&1; rc=$?
  p=$(grep -h '^test result' "$log" | awk '{p+=$4;f+=$6} END{print p+0"\t"f+0}')
  printf "%s\t%s\t%s\t%s\t%s\n" "$name" "$rc" "$p" "$(grep -c 'error: could not compile' "$log")" "$(( $(date +%s)-s ))s" >> "$G/$tag-summary.tsv"
done < "$2"
echo ALLDONE >> "$G/$tag-summary.tsv"
