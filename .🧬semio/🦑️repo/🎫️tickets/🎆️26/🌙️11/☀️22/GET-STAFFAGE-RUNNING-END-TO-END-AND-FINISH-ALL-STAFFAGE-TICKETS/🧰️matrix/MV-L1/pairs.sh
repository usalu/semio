#!/bin/zsh
# 👀️ Renders the line view of the product sheet (side, front, plan; the r3-2d tool of the 3D ticket) before and after a repair and stacks the two (before on top) inside ONE reservation of the memory queue: every line of the list is `<id> <pose or - for the default pose> <scale>…`; "before" is `$GEN/backup/<id>.json`, "after" the catalog file or, with TAG=try, `$GEN/try/<id>.json`. The pictures land in `$GEN/sheets/<id>.s<scale>.<pose>.<after|try>.png`. pairs.sh <megabytes> <list file>
source "${0:A:h}/env.sh"
mb="$1"; list="$2"; mkdir -p "$GEN/sheets" "$GEN/log"; export MEMLOG="$GEN/log/mem-sheets.txt"; export TAG="${TAG:-after}"
cd "$R" && INSIDE=1 repo capacity run --memory="$mb" -- zsh -c '
while read -r a pose scales; do
  [[ -z "$a" ]] && continue
  for tag in before "$TAG"; do
    work="$GEN/sheets/work-$tag/$a"; mkdir -p "$work"
    case "$tag" in before) cp "$GEN/backup/$a.json" "$work/🔣️.json";; try) cp "$GEN/try/$a.json" "$work/🔣️.json";; *) cp "$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)" "$work/🔣️.json";; esac
  done
  [[ "$pose" == "-" ]] && pose="$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))[\"asset\"][\"presentation\"][\"defaultPose\"])" "$GEN/sheets/work-before/$a/🔣️.json")"
  for s in ${=scales}; do
    for tag in before "$TAG"; do
      png="$("$W/q.sh" 0 "$T14" 900 "$B" r3-2d.ts --file "$GEN/sheets/work-$tag/$a/🔣️.json" --scale "$s" --pose "$pose" --views light-line 2>&1 | tail -1)"
      [[ -f "$png" ]] && mv "$png" "$GEN/sheets/work-$tag/$a.png" || echo "no sheet: $png"
    done
    "$W/q.sh" 0 "$T14" 300 "$B" "$W/pair.ts" "$GEN/sheets/work-before/$a.png" "$GEN/sheets/work-$TAG/$a.png" "$GEN/sheets/$a.s$s.$pose.$TAG.png" 2>&1 | tail -1
  done
  rm -rf "$GEN/sheets/work-before" "$GEN/sheets/work-$TAG"
done < "$1"' pairs "$list"
