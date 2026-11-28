#!/bin/zsh
# 🍋️ The ops of `small-leaved-lime` from the thinning analysis of this moment and its trial numbers inside one grant: lime.sh [thin options of the body…] runs `thin.ts` over the body and over the structure of the file `make.sh` would start from, joins the `thin` ops with the decisions (`ops/small-leaved-lime.text.json` when it exists) into `ops/small-leaved-lime.json`, makes the trial copy and measures it (`trial.sh`), then the GLB of every pose of the file before and of the trial copy with the writer of this moment (`sizes.sh`).
source "${0:A:h}/env.sh"
a=small-leaved-lime; f="$(ls "$C"/*/"$a"/🔣️.json)"; src="$a"
[[ -f "$GEN/written/$a.sha" && "$(shasum -a 256 < "$f" | cut -c1-16)" == "$(cat "$GEN/written/$a.sha")" ]] && src="$GEN/backup/$a.json"
zsh "$W/thin.sh" "$a" "$src" --emitters '^th-transpiration-' --error "${ERR:-2}" --share "${SHARE:-0.25}" "$@"
zsh "$W/thin.sh" "$a-structure" "$src" --list structure --match '^member-' --share "${SHARE:-0.25}"
t=(); [[ -f "$W/ops/$a.text.json" ]] && t=("$W/ops/$a.text.json")
python3 "$W/join.py" "$a" "$GEN/thin/$a.json" "$GEN/thin/$a-structure.json" $t
zsh "$W/trial.sh" "$a"
zsh "$W/sizes.sh" "$GEN/backup/$a.json" "$GEN/try/$a.json"
