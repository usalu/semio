#!/bin/zsh
# usage: pgo.sh <file> <fnName|@line> [max] [apply] : auto phase split of one function (psplit2 copy), then tsdiff + shape + fnlen
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
T=$C/🗑️generated/fnsplit4a-census
cd /Users/ueli/Documents/semio
F="$1"; EXT="${1##*.}"; N=$(echo "$F:$2" | md5 | cut -c1-8)
cp "$F" $T/orig/p-$N.$EXT
node $T/autop.cjs "$F" "$2" $T/specs/p-$N.json "${3:-150}" > $T/specs/p-$N.txt 2>&1 || { cat $T/specs/p-$N.txt | head -5; exit 1; }
head -1 $T/specs/p-$N.txt
node $T/psplit2.cjs $T/specs/p-$N.json > $T/specs/p-$N.out 2>&1
E_=$(node -e 'const o=require("fs").readFileSync(process.argv[1],"utf8");const i=o.indexOf("\"errors\"");console.log(i<0?o.slice(0,500):o.slice(i).replace(/\s+/g," ").slice(0,700))' $T/specs/p-$N.out)
echo "spec p-$N: $E_"
if [[ "$4" == apply && "$E_" == *'"errors": []'* ]]; then
  node $T/psplit2.cjs $T/specs/p-$N.json APPLY 2>&1 | tail -1
  if [[ "$EXT" == tsx ]]; then export TSDIFF_REACT=1; fi
  node $T/tsdiff.cjs "$F" $T/orig/p-$N.$EXT --spec $T/specs/p-$N.json 2>&1 | tail -5 | cut -c1-300
  A=$(node $T/titles.cjs $T/orig/p-$N.$EXT "${2#@}" 2>&1 | md5); B=$(node $T/titles.cjs "$F" "${2#@}" 2>&1 | md5)
  [[ "$A" == "$B" ]] && echo "TITLES-IDENTICAL" || echo "TITLES-DIFFER(or not a registration)"
  D="${F%/*}"
  node $C/fnsplit3-tools/fnlen.cjs "$F" "$D"/*/🟦️.$EXT 2>&1 | sort -n | tail -2
fi
