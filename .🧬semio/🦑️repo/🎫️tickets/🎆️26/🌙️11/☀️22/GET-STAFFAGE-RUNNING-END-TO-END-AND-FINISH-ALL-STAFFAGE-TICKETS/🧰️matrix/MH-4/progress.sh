#!/bin/zsh
# 📝️ Keeps the interim report current while the closing proofs run: progress.sh   appends to U/multiview-MH-4.md one line per asset whose `light` part (gate, scan, measure, check, matrix) or `suites` part has ended, with its numbers, until $GEN/final.log holds "suites END"; it stops as soon as the first line of the report is no longer "IN WORK".
source "${0:A:h}/env.sh"
seen=0
while true; do
  [[ "$(head -1 "$U/multiview-MH-4.md")" == "IN WORK" ]] || exit 0
  n=$(grep -c "" "$GEN/final.log" 2>/dev/null || echo 0)
  if (( n > seen )); then
    sed -n "$((seen+1)),${n}p" "$GEN/final.log" | while read -r t part a; do
      case "$part" in
        light) echo "- $t \`$a\` on the engine of 11:10: $(grep -E "^gate|^scan|^check" "$GEN/proof/$a.light.out" | cut -c1-90 | tr '\n' ';') $(head -1 "$GEN/proof/$a.matrix.out" 2>/dev/null | cut -c1-80)" >> "$U/multiview-MH-4.md" ;;
        suites) echo "- $t \`$a\` suites: $(cut -c1-70 "$GEN/proof/$a.suites.out" | tr '\n' ';')" >> "$U/multiview-MH-4.md" ;;
      esac
    done
    seen=$n
  fi
  grep -q "suites END" "$GEN/final.log" 2>/dev/null && exit 0
  sleep 30
done
