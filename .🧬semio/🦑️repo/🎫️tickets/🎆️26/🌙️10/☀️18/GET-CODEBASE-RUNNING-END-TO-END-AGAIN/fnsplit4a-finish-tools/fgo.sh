#!/bin/zsh
# usage: fgo.sh "<relfile>|<fnName>|<line>|<lines>" [max=150] [apply] : auto phase split of one function inside the write-isolated overlay (never the live tree), then tsdiff + titles + fnlen; reverts inside the overlay unless tsdiff is clean
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
TD=$C/fnsplit4a-finish-tools
OV=/private/tmp/fnsplit4a-finish/ov
W=/private/tmp/fnsplit4a-finish/work
mkdir -p $W/orig $W/specs $W/logs
cd $OV || exit 1
export PS_OVERLAY=$OV PS_REAL=/Users/ueli/Documents/semio
IFS='|' read -r F NM LN LNS <<< "$1"
MAX="${2:-150}"
N=$(echo "$F:$NM" | md5 | cut -c1-8); EXT="${F##*.}"; D="${F%/*}"
SNAP=$W/orig/p-$N.$EXT
[[ -f $SNAP ]] || cp "$F" $SNAP
[[ "$(cmp -s $SNAP "$F"; echo $?)" == 0 ]] || { echo "[$NM] overlay file differs from snapshot (already split?)"; exit 2; }
cp $SNAP "$D/__orig_$N.$EXT"
node $TD/autop.cjs "$F" "$NM" $W/specs/p-$N.json "$MAX" > $W/specs/p-$N.txt 2>&1 || { head -3 $W/specs/p-$N.txt; rm -f "$D/__orig_$N.$EXT"; exit 1; }
head -1 $W/specs/p-$N.txt
[[ -n "$FGO_EXP" ]] && node -e 'const fs=require("fs");const s=JSON.parse(fs.readFileSync(process.argv[1],"utf8"));s.allowExportedMoves=true;fs.writeFileSync(process.argv[1],JSON.stringify(s,null,1))' $W/specs/p-$N.json
node $TD/psplit2.cjs $W/specs/p-$N.json > $W/specs/p-$N.out 2>&1
E_=$(node -e 'const o=require("fs").readFileSync(process.argv[1],"utf8");const i=o.indexOf("\"errors\"");console.log(i<0?o.slice(0,500):o.slice(i).replace(/\s+/g," ").slice(0,900))' $W/specs/p-$N.out)
echo "spec p-$N: $E_"
if [[ "$3" == apply && "$E_" == *'"errors": []'* ]]; then
  node $TD/psplit2.cjs $W/specs/p-$N.json APPLY 2>&1 | tail -1
  [[ "$EXT" == tsx ]] && export TSDIFF_REACT=1
  CONS=""
  if [[ -n "$FGO_EXP" ]]; then
    CLF=$W/specs/p-$N.cons; : > $CLF
    node $TD/fcons.cjs $W/specs/p-$N.json $SNAP --write $CLF 2>&1 | grep -v "^overlay " | cut -c1-400
    CONS=$(paste -sd';' $CLF)
  fi
  export TSDIFF_ALSO="$CONS"
  TD_OUT=$(TSDIFF_ALSO="$TSDIFF_ALSO" TSDIFF_TYPES=${TSDIFF_TYPES:-node,bun} TSDIFF_REPO=$OV TSDIFF_AT=1 node $TD/tsdiff.cjs "$F" $SNAP --spec $W/specs/p-$N.json 2>&1 | tail -6 | cut -c1-300)
  print -r -- "$TD_OUT"
  A=$(node $TD/titles.cjs "$D/__orig_$N.$EXT" "$NM" 2>&1 | md5); B=$(node $TD/titles.cjs "$F" "$NM" 2>&1 | md5)
  [ "$A" = "$B" ] && echo "TITLES-IDENTICAL" || echo "TITLES-DIFFER(or not a registration)"
  node $TD/fnlen.cjs "$F" "$D"/*/🟦️.$EXT 2>&1 | sort -n | tail -3
  if [[ "$TD_OUT" != *"TSDIFF CLEAN"* ]]; then
    [[ -n "$CLF" ]] && while IFS= read -r c; do [[ -n "$c" ]] && cp "/Users/ueli/Documents/semio/$c" "$OV/$c"; done < $CLF
    node -e '
const fs=require("fs"),path=require("path");const s=JSON.parse(fs.readFileSync(process.argv[1],"utf8"));
const base=path.dirname(s.file);const dirs=[...(s.files||[]).map(f=>f.dir),...(s.groups||[]).map(f=>f.dir),...(s.context?[s.context.dir]:[]),...(s.shared?[s.shared.dir]:[]),...(s.helpers?[s.helpers.dir]:[])];
for(const d of dirs){fs.rmSync(path.join(base,d),{recursive:true,force:true});}
fs.copyFileSync(process.argv[2],s.file);console.log("REVERTED (tsdiff not clean)",dirs.length);' $W/specs/p-$N.json $SNAP
  fi
fi
rm -f "$D/__orig_$N.$EXT"
