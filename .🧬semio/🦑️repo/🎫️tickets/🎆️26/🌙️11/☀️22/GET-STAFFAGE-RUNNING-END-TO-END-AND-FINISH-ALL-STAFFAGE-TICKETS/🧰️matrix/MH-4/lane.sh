#!/bin/zsh
# 🛤️ A lane of jobs.sh that asks the queue again when the wait ends without a grant: lane.sh <MB>   ends when the lane was served (it ended its idle time) or after 8 waits.
source "${0:A:h}/env.sh"
for attempt in {1..8}; do
  rm -f "$GEN/lane.$1"; start=$(date +%s)
  GRANT="$1" SEMIO_LANGUAGE=en "$W/g.sh" "$W/jobs.sh" 30000
  [[ -f "$GEN/lane.$1" ]] && break
done
echo "lane $1 ended $(date +%H:%M) after $attempt request(s)"
