#!/bin/zsh
# 🔍️ Large single panels of product sheets (`zoom.ts`) inside ONE reservation of the memory queue: every line of the list is `<id> <tag: after|before|try> <pose> <scale> <elevation|front|plan> [height in pixels]`; the pictures land in `$GEN/sheets/z.<id>.s<scale>.<pose>.<viewpoint>.<tag>.png`. zooms.sh <megabytes> <list file>
source "${0:A:h}/env.sh"
mb="$1"; list="$2"; mkdir -p "$GEN/sheets" "$GEN/log"; export MEMLOG="$GEN/log/mem-sheets.txt"
cd "$R" && INSIDE=1 repo capacity run --memory="$mb" -- zsh -c '
while read -r a tag pose s view height; do
  [[ -z "$a" ]] && continue
  case "$tag" in before) f="$GEN/backup/$a.json";; try) f="$GEN/try/$a.json";; *) f="$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)";; esac
  "$W/q.sh" 0 "$T14" 600 "$B" "$W/zoom.ts" "$f" "$s" "$pose" "$view" "$GEN/sheets/z.$a.s$s.$pose.$view.$tag.png" ${height:-1500} 2>&1 | tail -1
done < "$1"' zooms "$list"
