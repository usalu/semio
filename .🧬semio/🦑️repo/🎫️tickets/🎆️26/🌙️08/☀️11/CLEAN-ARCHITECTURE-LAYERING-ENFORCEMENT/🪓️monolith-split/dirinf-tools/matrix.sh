#!/bin/zsh
T=${0:A:h}
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/dirinf-ws
SPLIT=$1; ORIG=${2:-$S/sbx-orig}
OUT=$S/orc
for k in native test feature testfeature; do
  case $k in
    native) A=();;
    test) A=(--test);;
    feature) A=(--cfg 'feature="integration-fixtures"');;
    testfeature) A=(--test --cfg 'feature="integration-fixtures"');;
  esac
  if [[ -n $FORCE_ORIG || ! -s $OUT/o-$k.norm ]]; then $T/oracle.sh $ORIG o-$k $A; fi
  $T/oracle.sh $SPLIT s-$k $A
  echo "-- $k"
  python3 $T/cmp.py $OUT/o-$k.norm $OUT/s-$k.norm '📇️directory' '💡️inference/🏃️runtime' | /usr/bin/grep -v 'aborting due'
done
