#!/bin/zsh
# 🌰️ The ops of `english-oak` from the thinning analysis of this moment and its trial numbers inside one grant: oak.sh [thin options of the body…] runs `thin.ts` over the body and over the members of the structure of the file `make.sh` would start from (`ERR` mm of paper and `SHARE` of the radius, default 1 and 0.1), joins the `thin` ops with the decisions (`ops/english-oak.text.json` when it exists) into `ops/english-oak.json`, makes the trial copy and measures it (`trial.sh`), then the GLB of every pose of the trial copy with the writer of this moment (`sizes.sh`).
source "${0:A:h}/env.sh"
a=english-oak; f="$(ls "$C"/*/"$a"/🔣️.json)"; src="$a"
[[ -f "$GEN/written/$a.sha" && "$(shasum -a 256 < "$f" | cut -c1-16)" == "$(cat "$GEN/written/$a.sha")" ]] && src="$GEN/backup/$a.json"
zsh "$W/thin.sh" "$a" "$src" --emitters '^th-transpiration-' --error "${ERR:-1}" --share "${SHARE:-0.1}" "$@"
zsh "$W/thin.sh" "$a-structure" "$src" --list structure --match '^member-' --error "${ERR:-1}" --share "${SHARE:-0.1}"
t=(); [[ -f "$W/ops/$a.text.json" ]] && t=("$W/ops/$a.text.json")
python3 "$W/join.py" "$a" "$GEN/thin/$a.json" "$GEN/thin/$a-structure.json" $t
zsh "$W/trial.sh" "$a"
zsh "$W/sizes.sh" "$GEN/try/$a.json"
