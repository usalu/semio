#!/bin/zsh
# usage: kgo.sh <file> <fnName> [max] [apply] : auto ksplit2 of a registration function (tries without, then with helpers); verifies the title sequence
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
T=$C/🗑️generated/fnsplit4a-census
cd /Users/ueli/Documents/semio
F="$1"; EXT="${1##*.}"; N=$(echo "$F:$2" | md5 | cut -c1-8)
cp "$F" $T/orig/k-$N.$EXT
for H in 0 1; do
  node $T/autok.cjs "$F" "$2" $T/specs/k-$N.json "${3:-150}" $H > $T/specs/k-$N.txt 2>&1 || { head -3 $T/specs/k-$N.txt; exit 1; }
  ORIG_TEXT= node $C/fnsplit2-tools/ksplit2.cjs $T/specs/k-$N.json > $T/specs/k-$N.out 2>&1
  E_=$(node -e 'const o=require("fs").readFileSync(process.argv[1],"utf8");const i=o.indexOf("\"errors\"");console.log(i<0?o.slice(0,500):o.slice(i).replace(/\s+/g," ").slice(0,700))' $T/specs/k-$N.out)
  [[ "$E_" == *'"errors": []'* ]] && break
done
echo "[$2] spec k-$N helpers=$H groups=$(wc -l < $T/specs/k-$N.txt): $E_"
if [[ "$4" == apply && "$E_" == *'"errors": []'* ]]; then
  node $C/fnsplit2-tools/ksplit2.cjs $T/specs/k-$N.json APPLY 2>&1 | tail -1
  if [[ "$EXT" == tsx ]]; then export TSDIFF_REACT=1; fi
  node $T/tsdiff.cjs "$F" $T/orig/k-$N.$EXT --spec $T/specs/k-$N.json 2>&1 | tail -4 | cut -c1-300
  A=$(node $T/titles.cjs $T/orig/k-$N.$EXT "$2" 2>&1 | md5); B=$(node $T/titles.cjs "$F" "$2" 2>&1 | md5)
  [[ "$A" == "$B" ]] && echo "TITLES-IDENTICAL" || echo "TITLES-DIFFER"
  D="${F%/*}"
  $C/fnsplit2-tools/shape.sh "$D" 2>&1 | grep -v "^RGI\|error:\|^$\|Bun v\|ok$\|problems: 0\|bad=0" | tail -4 | cut -c1-200
  node $C/fnsplit3-tools/fnlen.cjs "$F" "$D"/*/🟦️.$EXT "$D"/*/*/🟦️.$EXT 2>&1 | sort -n | tail -2
fi
