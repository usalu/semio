#!/bin/zsh
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test" || exit 1
OUT="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w14-ooxml-cad-parity/📊️baseline.txt"
: > "$OUT"
for c in mutate-pptx-ecma-376 mutate-xlsx-ecma-376 mutate-bcf-2-1 mutate-ifc-2x3 mutate-ifc-4 mutate-step-ap214 mutate-dxf-r12; do
  echo "===== SUBJECT $c =====" >> "$OUT"
  bun ./📜️script.ts subject exhaustive --owner 🗄️stdio --case "$c" >> "$OUT" 2>&1
  echo "exit=$?" >> "$OUT"
  echo "===== PARITY $c =====" >> "$OUT"
  bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case "$c" >> "$OUT" 2>&1
  echo "exit=$?" >> "$OUT"
done
echo "ALLDONE" >> "$OUT"
