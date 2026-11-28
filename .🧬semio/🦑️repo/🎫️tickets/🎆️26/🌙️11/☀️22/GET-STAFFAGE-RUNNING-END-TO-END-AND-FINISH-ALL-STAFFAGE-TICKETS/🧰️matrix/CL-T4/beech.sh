#!/bin/zsh
# 🌳️ The ops of the beech from the crown analysis of this moment and its trial numbers inside one grant: beech.sh <segments of the coarse corner points> runs `crown.ts`, joins its `keep` ops before `ops/european-beech.base.json` and the decisions (`ops/european-beech.text.json` when it exists) and runs `trial.sh`.
source "${0:A:h}/env.sh"
zsh "$W/crown.sh" rims european-beech --segments "$1"; grep "^1:" "$GEN/crown/rims.txt" | cut -c1-330
t=(); [[ -f "$W/ops/european-beech.text.json" ]] && t=("$W/ops/european-beech.text.json")
python3 "$W/join.py" european-beech "$GEN/crown/rims.json" "$W/ops/european-beech.base.json" $t
zsh "$W/trial.sh" european-beech
