#!/bin/zsh
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R147
D=$(dirname $0)
for f in $(ls $S/tests | grep '^b[0-9]'); do
 for mode in base new; do
  if [ $mode = base ]; then export R147_BASE=1; else unset R147_BASE; fi
  $D/run-staged.sh $f > $S/bat-$f-$mode.txt 2>&1
  echo "$f $mode: $(grep -E '^ *[0-9]+ (pass|fail)' $S/bat-$f-$mode.txt | tr '\n' ' ')"
 done
done
