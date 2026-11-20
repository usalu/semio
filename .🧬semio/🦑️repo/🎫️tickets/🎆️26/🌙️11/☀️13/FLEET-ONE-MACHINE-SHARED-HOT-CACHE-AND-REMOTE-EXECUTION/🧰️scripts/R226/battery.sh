#!/bin/zsh
# usage: battery.sh <mode> <from> <to> ; appends "mode<TAB>dir<TAB>pass<TAB>fail" to results.tsv
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/60403654-b0e9-4c0d-a7d3-fba5340a3de9/scratchpad/R226
sed -n "$2,$3p" $S/battery.txt | while read -r rel; do
 d="/Users/ueli/Documents/semio/🧰️framework/$rel"; [ -d "$d" ] || d="/Users/ueli/Documents/semio/$rel"
 [ -f "$d/🟦️.ts" ] || continue
 out=$($S/run.sh "$d" $1 2>&1)
 p=$(echo "$out"|grep -E '^ *[0-9]+ pass'|awk '{print $1}'); f=$(echo "$out"|grep -E '^ *[0-9]+ fail'|awk '{print $1}')
 printf '%s\t%s\t%s\t%s\n' "$1" "${rel##*🏃️process/}" "${p:-?}" "${f:-?}" >> $S/results.tsv
done
