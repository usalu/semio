#!/bin/zsh
# 🧵️ The trial copy of `english-oak` from the thinning analysis already in `$GEN/thin` (of `oak.sh`), inside the grant of its caller: joins the ops with the decisions, makes the trial copy and measures it (`trial.sh`), then the GLB of every pose (`sizes.sh`).
source "${0:A:h}/env.sh"
a=english-oak; t=(); [[ -f "$W/ops/$a.text.json" ]] && t=("$W/ops/$a.text.json")
python3 "$W/join.py" "$a" "$GEN/thin/$a.json" "$GEN/thin/$a-structure.json" $t
zsh "$W/trial.sh" "$a"
zsh "$W/sizes.sh" "$GEN/try/$a.json"
