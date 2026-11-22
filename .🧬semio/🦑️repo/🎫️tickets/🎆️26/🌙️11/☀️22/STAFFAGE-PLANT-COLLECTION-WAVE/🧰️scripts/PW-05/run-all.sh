#!/bin/zsh
# 🔁 PW-05: rebuild the named species (default all three) and validate each candidate with schema, validateAsset and the volume rules; one tool process at a time. PRINT=" " includes the print checks.
H="${0:A:h}"
V="$H/../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
G="$H/../../🗑️generated/PW-05"
list=("$@")
(( ${#list} == 0 )) && list=(tulip-tree red-oak black-locust)
for s in $list; do
  (cd "$H" && repo tools exec bun -- assemble.ts $s 2>&1 | grep -v "still working\|capacity\|tools\]" > "$G/build-$s.txt")
  (cd "$V" && repo tools exec bun -- r3-validate.ts --file "$G/$s/🔣️.json" --volumes ${=PRINT:---no-print} 2>&1 | grep -v "still working\|capacity\|tools\]" | cut -c1-500 > "$G/validate-$s.txt")
  echo "[DEBUG] $s: $(tail -1 $G/validate-$s.txt | cut -c1-200)"
done
