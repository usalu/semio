#!/bin/zsh
# 🖊️ Writes the volumes into the catalog documents, one after the other, each read right before it is written: the s4 script (refuses another authored volume without --force), then the joint and trumpet repairs of other scales; prints per asset which JSON paths below `asset` differ from the backup: apply.sh [<id>…]
me="${0:A}"; source "${0:A:h}/env.sh"
if [[ "$1" != "--inside" ]]; then cd "$R" && SEMIO_LANGUAGE=en exec repo capacity run --memory=800 -- timeout 1800 "$me" --inside "$@"; fi
shift; ids=("$@"); [[ ${#ids} -eq 0 ]] && ids=($IDS)
for a in $ids; do
  f="$(asset "$a")"
  cmp -s "$f" "$GEN/backup/$a.json" && same="as backed up" || same="CHANGED SINCE THE BACKUP"
  echo "$same $("$B" "$S4/$a.ts" "$f" $FORCE 2>&1 | grep -oE '1:.*(written|unchanged)|refused.*' | head -1)"
  case $a in used-cast-iron-sectional-boiler|used-panel-radiator-pallet|used-moving-boxes) python3 -I "$W/joints.py" "$f";; used-trumpet) python3 -I "$W/trumpet.py" "$f";; esac
  python3 -I "$W/paths.py" "$GEN/backup/$a.json" "$f"
done
