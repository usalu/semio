#!/bin/zsh
K=$(cd $(dirname $0); pwd); S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R202
PN=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes
cd $PN/🏊️project-farm/🧪️tests && FARM=(${(f)"$(ls -d */ | sed 's#/$##')"})
DIRS=()
for d in $FARM; do [ -f "$PN/🏊️project-farm/🧪️tests/$d/🟦️.ts" ] && DIRS+=("$PN/🏊️project-farm/🧪️tests/$d"); done
DIRS+=("$PN/🏊️project-farm/🧪️tests" "$PN/📥️take/🧪️tests" "$PN/📥️take/🧪️tests/📣️published" "$PN/🧪️tests/📥️take" "$PN/🧪️tests/🧾️poll-ledger" "$PN/🧪️tests/🫥️self-progress" "$PN/🔭️look-queue/🧪️tests" "$PN/🧪️tests/🔭️look-job" "$PN/🌐️owner-pool/🧪️tests/🌊️early-wave")
for d in $DIRS; do [ -f "$d/🟦️.ts" ] || continue; out=$($K/t.sh "$d"); echo "$d: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"; echo "$out"|grep -E '^\(fail\)'; done
echo ALLDONE
