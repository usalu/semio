#!/bin/zsh
# ⚖️ Both dry runs of several assets in ONE grant of the memory queue, nothing of the catalog is written: the nested-prism chain (`ncampaign.sh`) and the single-solid chain of the first pass (`try.sh <id> take`: rebuild from `ops/<id>.json`, gate, measure), so that the better proven copy per asset can be chosen; the peak memory of each part is printed (`/usr/bin/time -l`): both.sh <id>…
BATCH=MA source "${0:A:h}/../MR/env.sh"
for a in "$@"; do
  /usr/bin/time -l "$W/ncampaign.sh" "$a" 2> "$GEN/nest/$a.time" ; echo "  nest peak $(( $(grep 'maximum resident' "$GEN/nest/$a.time" | awk '{print $1}') / 1048576 )) MB"
  /usr/bin/time -l "$W/try.sh" "$a" take > "$GEN/gate/$a.take.out" 2> "$GEN/gate/$a.take.time"; echo "$a fit chain: $(tail -1 "$GEN/gate/$a.take.log") | $(grep -E 'silhouette:|validate:|sweep:' "$GEN/gate/$a.take.out" | tail -3 | tr '\n' ';') peak $(( $(grep 'maximum resident' "$GEN/gate/$a.take.time" | awk '{print $1}') / 1048576 )) MB"
done
