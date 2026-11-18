#!/bin/zsh
# batch2.sh outfile files... (unique names by path hash)
O=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R153
out=$1; shift
for f in "$@"; do h=$(echo -n "$f" | md5 | cut -c1-6); $O/run.sh "$f" > $O/o2-$h.txt 2>&1; echo "$h ${f:h:t}: $(grep -E '^ *[0-9]+ (pass|fail)' $O/o2-$h.txt | tr '\n' ' ')" >> $out; done
echo DONE >> $out
