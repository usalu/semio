#!/bin/zsh
# usage: runpos.sh <label> [overlay]   runs the three family oracles (lib,test) sequentially; overlay = stage dir for the post run
cd "$(dirname "$0")"
L=$1
for k in p b s; do
  if [ -n "$2" ]; then export C4P28_OVERLAY="$2"; else unset C4P28_OVERLAY; fi
  POS_PREFIX=c4p28 perl -e 'alarm 2400; exec @ARGV' -- python3 posrun.py run c4p28$k.json --cfgs ${CFGS:-lib,test} --label ${L}$k --no-base > ../🗑️generated/c4p28-pos-${L}$k.log 2>&1
done
echo done > ../🗑️generated/c4p28-pos-${L}.done
