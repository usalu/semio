#!/bin/zsh
# usage: fverify.sh : strong re-verification of every applied phase split in the overlay (types node,bun; consumers included): tsdiff + shape + sizes
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
TD=$C/fnsplit4a-finish-tools; OV=/private/tmp/fnsplit4a-finish/ov; W=/private/tmp/fnsplit4a-finish/work
cd $OV
for J in $W/specs/p-*.json; do
  ID=${${J:t}#p-}; ID=${ID%.json}
  F=$(node -e 'console.log(require("path").relative(process.argv[1],JSON.parse(require("fs").readFileSync(process.argv[2],"utf8")).file))' $OV $J)
  EXT=${F##*.}; SNAP=$W/orig/p-$ID.$EXT
  [[ -f $SNAP ]] || continue
  cmp -s $SNAP "$F" && continue
  CONS=""; [[ -s $W/specs/p-$ID.cons ]] && CONS=$(paste -sd';' $W/specs/p-$ID.cons)
  [[ $EXT == tsx ]] && export TSDIFF_REACT=1 || unset TSDIFF_REACT
  R=$(TSDIFF_ALSO="$CONS" TSDIFF_TYPES=node,bun TSDIFF_REPO=$OV node $TD/tsdiff.cjs "$F" $SNAP --spec $J 2>&1 | tail -4 | cut -c1-200)
  V=$(print -r -- "$R" | grep -c "TSDIFF CLEAN")
  print -r -- "$ID ${${F:h}:t} tsdiff=$V $(print -r -- "$R" | head -1)"
done
