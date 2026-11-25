#!/bin/zsh
# 🖼️ Renders the sheets of several plants of batch MV-3, one after the other (meant for one grant: `grant.sh 2500 <timeout> look-many.sh <tags> <id:scale:pose>…`): for every `<id>:<scale>:<pose>` the sheet of each tag of the comma list `<tags>` (before, try, after) with `look.sh`; `default` as pose is the default pose of the asset.
export BATCH=MV-3; source "${0:A:h}/../MR/env.sh"
tags=(${(s:,:)1}); shift
for item in "$@"; do
  p=(${(s.:.)item}); for tag in $tags; do
    if [[ "$p[3]" == default ]]; then TAG=$tag "$W/look.sh" "$p[1]" "$p[2]"; else TAG=$tag POSE="$p[3]" "$W/look.sh" "$p[1]" "$p[2]"; fi
  done
done 2>&1 | tail -n 200
echo "looks done"
