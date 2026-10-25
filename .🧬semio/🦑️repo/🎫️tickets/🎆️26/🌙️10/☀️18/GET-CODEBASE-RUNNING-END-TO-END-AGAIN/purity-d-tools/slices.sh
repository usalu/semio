#!/bin/sh
# usage: slices.sh <tsv>  prints the finding count per product slice
P="$1"
for s in "🧰️framework/🛍️products/🦑️repo" "🧰️framework/🛍️products/📓️print" "🧰️framework/🛍️products/🎤️presentation" "🎓️teaching" "🧰️framework/🛍️products/📚️course" "♻️mit-bestand" "🧰️framework/🔨️modules/🏃️process" "🧰️framework/🛍️products/🐾️pets" "🧰️framework/🛍️products/❓️quiz" "🧰️framework/🛍️products/🖥️server"; do
  n=$(awk -F'\t' -v s="$s/" 'index($2,s)==1' "$P" | wc -l | tr -d ' '); echo "$n $s"
done
echo "$(wc -l < "$P" | tr -d ' ') total"
