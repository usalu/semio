#!/bin/zsh
# 👀️ The contact sheets g1..g4 of every candidate of the 27 assets at 1:200 and 1:500 (look.sh): look-all.sh
source "${0:A:h}/env.sh"
export CAND=1
for k in 0 1 2 3; do
  items=()
  for a in ${IDS[@]:$((k * 7)):7}; do items+=("$a:200" "$a:500"); done
  "$W/look.sh" "g$((k + 1))" $items
done
