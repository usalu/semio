#!/bin/zsh
# 👀️ Trial of a carrier on copies inside one reservation: `loft.ts` (or TOOL=slice) on every plant of ZIDS (result `$GEN/try/<id>-<tool>.json`), then every panel of the list file (`<id> <tag: before|slice|loft|after> <pose> <scale> <view> [height]`). look.sh <list file>   (ZIDS="a b", MB=1000, BODY=flags)
source "${0:A:h}/env.sh"
mkdir -p "$GEN/try" "$GEN/log" "$GEN/sheets" "$W/ops"; export MEMLOG="$GEN/log/mem-look.txt"
cd "$R" && INSIDE=1 repo capacity run --memory="${MB:-1000}" -- zsh -c '
for a in ${=ZIDS}; do s=$(date +%s)
"$W/q.sh" 0 "$T" 3000 "$B" "$W/${TOOL:-loft}.ts" "$GEN/backup/$a.json" --save "$W/ops/$a.json" --out "$GEN/try/$a-${TOOL:-loft}.json" ${=BODY} > "$GEN/log/$a-${TOOL:-loft}.log" 2>&1; echo "$a ${TOOL:-loft} exit $? $(( $(date +%s)-s )) s"; done
[[ -n "$1" ]] && while read -r a tag pose s view height; do
  [[ -z "$a" ]] && continue
  case "$tag" in before) f="$GEN/backup/$a.json";; after) f="$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)";; *) f="$GEN/try/$a-$tag.json";; esac
  "$W/q.sh" 0 "$T14" 600 "$B" "$W/zoom.ts" "$f" "$s" "$pose" "$view" "$GEN/sheets/z.$a.s$s.$pose.$view.$tag.png" ${height:-1300} 2>&1 | tail -1
done < "$1"' look "$1"
