#!/bin/zsh
# usage: bgo.sh <file> <scopeLine> [max] [apply] : auto range extraction (bx copy) from the function-like at scopeLine
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
T=$C/🗑️generated/fnsplit4a-census
cd /Users/ueli/Documents/semio
F="$1"; EXT="${1##*.}"; N=$(echo "$F:$2" | md5 | cut -c1-8)
cp "$F" $T/orig/b-$N.$EXT
node $T/autob.cjs "$F" "$2" $T/specs/b-$N.json "${3:-150}" > $T/specs/b-$N.txt 2>&1 || { head -5 $T/specs/b-$N.txt; exit 1; }
head -1 $T/specs/b-$N.txt
TS_TYPES=node node $T/bx.cjs $T/specs/b-$N.json > $T/specs/b-$N.out 2>&1
E_=$(node -e 'const o=require("fs").readFileSync(process.argv[1],"utf8");const i=o.indexOf("\"errors\"");console.log(i<0?o.slice(0,600):o.slice(i).replace(/\s+/g," ").slice(0,900))' $T/specs/b-$N.out)
echo "spec b-$N: $E_"
if [[ "$4" == apply && "$E_" == *'"errors": []'* ]]; then
  TS_TYPES=node node $T/bx.cjs $T/specs/b-$N.json APPLY 2>&1 | tail -1
  if [[ "$EXT" == tsx ]]; then export TSDIFF_REACT=1; fi
  D="${F%/*}"
  CH=$(node -e 'const s=require("./'"$T"'/specs/b-'"$N"'.json");const p=require("path");console.log(s.files.map(f=>p.join(p.dirname(s.file),f.dir,"🟦️."+"'"$EXT"'")).join(" "))')
  node $T/tsdiff.cjs "$F" $T/orig/b-$N.$EXT ${=CH} 2>&1 | tail -4 | cut -c1-300
  $C/fnsplit2-tools/shape.sh "$D" 2>&1 | grep -v "^RGI\|error:\|^$\|Bun v\|ok$\|problems: 0\|bad=0" | tail -5 | cut -c1-200
  node $C/fnsplit3-tools/fnlen.cjs "$F" ${=CH} 2>&1 | sort -n | tail -2
fi
