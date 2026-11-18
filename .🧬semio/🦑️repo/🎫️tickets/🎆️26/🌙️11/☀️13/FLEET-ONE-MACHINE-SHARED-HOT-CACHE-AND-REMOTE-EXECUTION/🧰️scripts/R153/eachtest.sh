#!/bin/zsh
cd /private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R153
T=$1; N=$2; OUT=$3
while IFS= read -r name; do
  r=$(./run.sh "$T" "^${name}\$" 2>&1)
  echo "$(echo "$r" | grep -E '^ *[0-9]+ (pass|fail)' | tr '\n' ' ') :: $name" >> $OUT
done < $N
echo DONE >> $OUT
