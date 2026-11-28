#!/bin/zsh
# 🌳️ The crown analysis of the three trees of lane CL-T5 inside the grant of its caller, each with the pattern of its foliage lobes and of its transpiration outlets: crowns.sh <segments> [id…] writes `$GEN/crown/<id>.json` and `.txt`; the source is the file `make.sh` would start from (the catalog file, or its backup when the catalog file is this lane's own write).
source "${0:A:h}/env.sh"
seg="$1"; shift; (( $# )) || set -- horse-chestnut small-leaved-lime sweetgum
typeset -A LOBES=(horse-chestnut '^lobe-' small-leaved-lime '^fol-' sweetgum '^(cl|m)-')
typeset -A OUTLETS=(horse-chestnut '^em-' small-leaved-lime '^th-transpiration-' sweetgum '^th-tp-')
for a in "$@"; do
  f="$(ls "$C"/*/"$a"/🔣️.json)"; src="$a"
  [[ -f "$GEN/written/$a.sha" && "$(shasum -a 256 < "$f" | cut -c1-16)" == "$(cat "$GEN/written/$a.sha")" ]] && src="$GEN/backup/$a.json"
  zsh "$W/crown.sh" "$a" "$src" --segments "$seg" --match "$LOBES[$a]" --emitters "$OUTLETS[$a]"
done
