#!/bin/zsh
# 🖼️ Renders product sheets (side, front, plan; line and solid view; the r3-2d tool of the 3D ticket) inside ONE reservation of the memory queue: every line of the list is `<id> <tag: after|before|try> <pose or - for the default pose> <scale>…`; `after` reads the catalog file, `before` `$GEN/backup/<id>.json`, `try` `$GEN/try/<id>.json`. The sheets land in `$GEN/sheets/<id>.s<scale>.<pose>.<tag>.png`. sheets.sh <megabytes> <list file>
source "${0:A:h}/env.sh"
mb="$1"; list="$2"; mkdir -p "$GEN/sheets" "$GEN/log"; export MEMLOG="$GEN/log/mem-sheets.txt"
cd "$R" && INSIDE=1 repo capacity run --memory="$mb" -- zsh -c '
while read -r a tag pose scales; do
  [[ -z "$a" ]] && continue
  work="$GEN/sheets/work-$tag-$$/$a"; mkdir -p "$work"
  case "$tag" in before) cp "$GEN/backup/$a.json" "$work/🔣️.json";; try) cp "$GEN/try/$a.json" "$work/🔣️.json";; *) cp "$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)" "$work/🔣️.json";; esac
  [[ "$pose" == "-" ]] && pose="$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))[\"asset\"][\"presentation\"][\"defaultPose\"])" "$work/🔣️.json")"
  for s in ${=scales}; do
    png="$("$W/q.sh" "$0" "$T14" 900 "$B" r3-2d.ts --file "$work/🔣️.json" --scale "$s" --pose "$pose" 2>&1 | tail -1)"
    [[ -f "$png" ]] && mv "$png" "$GEN/sheets/$a.s$s.$pose.$tag.png" && echo "$GEN/sheets/$a.s$s.$pose.$tag.png" || echo "no sheet: $png"
  done
  rm -rf "$GEN/sheets/work-$tag-$$"
done < "$1"' "$mb" "$list"
