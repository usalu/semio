#!/bin/zsh
# 🖼️ Renders the product sheet (side, front, plan) of every changed scale of the batch, in a pose that turns the changed parts (`sheets.txt`: <id>:<scale>:<pose>), once from the catalog documents (tag `now`) and once from the backups of the untouched documents (tag `was`), with look.sh, one process after the other: sheets.sh [now|was]…   (run it through q.sh with 2500 MB)
source "${0:h}/env.sh"
tags=("$@"); [[ ${#tags} -eq 0 ]] && tags=(now was)
items=($(cat "$W/sheets.txt"))
for tag in $tags; do
  src=catalog; [[ "$tag" == was ]] && src="$GEN/backup"
  SIZE=${SIZE:-1200} "$W/look.sh" "$tag" "$src" $items
done
