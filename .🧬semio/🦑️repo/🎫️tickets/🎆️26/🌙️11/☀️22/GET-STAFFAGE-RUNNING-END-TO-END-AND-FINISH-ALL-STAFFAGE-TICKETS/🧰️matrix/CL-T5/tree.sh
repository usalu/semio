#!/bin/zsh
# 🌳️ The ops of one tree from the crown analysis of this moment and its trial numbers inside one grant: tree.sh <id> <segments of the coarse corner points> runs `crowns.sh`, joins its `keep` ops with `ops/<id>.base.json` and the decisions (`ops/<id>.text.json` when it exists) into `ops/<id>.json`, makes the trial copy and measures it (`trial.sh`), then the GLB of every pose of the file before (`$GEN/backup/<id>.json`) and of the trial copy with the writer of this moment (`sizes.sh`).
source "${0:A:h}/env.sh"
a="$1"
zsh "$W/crowns.sh" "$2" "$a" | cut -c1-420
t=(); [[ -f "$W/ops/$a.text.json" ]] && t=("$W/ops/$a.text.json")
python3 "$W/join.py" "$a" "$GEN/crown/$a.json" "$W/ops/$a.base.json" $t
zsh "$W/trial.sh" "$a"
zsh "$W/sizes.sh" "$GEN/backup/$a.json" "$GEN/try/$a.json"
