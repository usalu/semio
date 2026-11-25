#!/bin/zsh
# 👀️ Candidates of the s4 scripts (copy of the catalog document with the script applied, $GEN/cand/<id>/🔣️.json) and their r3 sheets of the body at the next finer scale and at the written one, joined side by side into $GEN/sheets/<id>.<scale>.png, in one queue grant: sheets.sh [<id>…]   (LIST=thermal|structure for another list, CUT=1 for the cut)
source "${0:A:h}/env.sh"
me="${0:A}"; if [[ "$1" != "--inside" ]]; then cd "$R" && SEMIO_LANGUAGE=en exec repo capacity run --memory=800 -- timeout 1800 "$me" --inside "$@"; fi
shift; ids=("$@"); [[ ${#ids} -eq 0 ]] && ids=($IDS)
list="${LIST:-body}"; mkdir -p "$GEN/sheets"
cut=(); [[ -n "$CUT" ]] && cut=(--cut)
sheet() { (cd "$T14" && timeout 120 "$B" r3-sheet.ts --file "$GEN/cand/$1/🔣️.json" --scale "$2" --list "$list" --no-rig --no-labels $cut 2>&1 | grep -o '/.*\.png\|🗑️generated.*\.png\|NOT DRAWN.*\|rror.*' | tail -1); }
for a in $ids; do
  mkdir -p "$GEN/cand/$a"; cp "$(asset "$a")" "$GEN/cand/$a/🔣️.json"
  (cd "$R" && "$B" "$S4/$a.ts" "$GEN/cand/$a/🔣️.json" --force) | cut -c1-90
  pairs=("200 500"); [[ $a == used-microwave-oven ]] && pairs=("100 200" "200 500")
  for p in $pairs; do
    parts=()
    for s in ${=p}; do f="$(sheet "$a" "$s")"; [[ -f "$f" ]] || f="$T14/$f"; [[ -f "$f" ]] && { cp "$f" "$GEN/sheets/part.$s.png"; rm "$f"; parts+=("$GEN/sheets/part.$s.png"); } || echo "no sheet $a $s $f"; done
    "$B" "$W/join.ts" "$GEN/sheets/$a.${p##* }.$list.png" $parts && rm -f $parts
  done
done
