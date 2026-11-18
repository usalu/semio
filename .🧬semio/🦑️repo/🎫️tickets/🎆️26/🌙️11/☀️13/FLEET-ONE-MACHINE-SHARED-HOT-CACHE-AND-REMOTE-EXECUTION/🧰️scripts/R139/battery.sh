#!/bin/zsh
# usage: battery.sh <outfile> <base|stage> [parallel]; one line per test dir: dir pass fail
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R139
out=$1; mode=$2; par=${3:-3}
one(){ d=$1; o=$($SC/t.sh $mode "$d" 2>&1 | grep -E "^ *[0-9]+ (pass|fail)$|^\(fail\)" ); p=$(echo "$o" | awk '/ pass$/{print $1}'); f=$(echo "$o" | awk '/ fail$/{print $1}'); echo "$d ${p:-?} ${f:-?}"; echo "$o" | grep '^(fail)' | sed 's/^/   /' | cut -c1-160; }
while read d; do one "$d" & while [ $(jobs -r | wc -l) -ge $par ]; do sleep 1; done; done < $SC/tests.lst > $out; wait
