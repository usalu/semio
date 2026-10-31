#!/bin/bash
# usage: rend.sh id [scales...]  -> img/w-<id>-<s>.png from work json, montage
D=$(dirname "$0"); G="$D/../../🗑️generated/CVF-V"; id=$1; shift
S=${@:-"500 200 100 50"}; files=""
for s in $S; do px=$(python3 -c "print({500:16,200:10,100:6,50:3.5}[$s])"); python3 "$D/preview.py" "$G/work/$id.json" $s $s ${PX:-$px} "$G/img/w-$id-$s.png" >/dev/null; files="$files $G/img/w-$id-$s.png"; done
python3 "$D/montage.py" "$G/img/wm-$id.png" ${H:-700} $files
