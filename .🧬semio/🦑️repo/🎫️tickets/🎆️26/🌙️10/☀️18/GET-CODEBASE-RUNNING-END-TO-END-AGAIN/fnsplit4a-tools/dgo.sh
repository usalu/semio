#!/bin/zsh
# usage: dgo.sh <file> <fnStartLine> [max] [apply] : auto dsplit of the module-level describe holding the function starting at that line
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
T=$C/🗑️generated/fnsplit4a-census
cd /Users/ueli/Documents/semio
F="$1"; EXT="${1##*.}"; N=$(echo "$F" | md5 | cut -c1-8)
DLN=($(node $T/findtop.cjs "$F" "$2")); DL=$DLN[1]; NTH=${DLN[2]:-0}
if [[ "$DL" == none ]]; then echo "NOT-TOP-LEVEL-DESCRIBE"; exit 1; fi
cp "$F" $T/orig/d-$N.$EXT
node $T/autod.cjs "$F" $DL $T/specs/d-$N.json "${3:-165}" > $T/specs/d-$N.txt
node $C/fnsplit2-tools/dsplit.cjs $T/specs/d-$N.json > $T/specs/d-$N.out 2>&1
E_=$(node -e 'const o=require("fs").readFileSync(process.argv[1],"utf8");const i=o.indexOf("\"errors\"");console.log(o.slice(i).replace(/\s+/g," ").slice(0,600))' $T/specs/d-$N.out)
echo "spec d-$N: $(wc -l < $T/specs/d-$N.txt) parts; $E_"
if [[ "$4" == apply && "$E_" == *'"errors": []'* ]]; then
  node $C/fnsplit2-tools/dsplit.cjs $T/specs/d-$N.json APPLY 2>&1 | tail -1
  $T/dchk.sh "$F" $T/orig/d-$N.$EXT $NTH $T/specs/d-$N.json
fi
