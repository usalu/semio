#!/bin/zsh
# 🧪️ One body trial of `black-alder` with its light proof and the 1:50 `gale` panels inside one grant: trial (flags of `flags/black-alder.caps`), light proof, the copy kept as `$GEN/body/black-alder.json`, front and plan panels with the tag given. alder-body.sh <tag>
source "${0:A:h}/env.sh"
zsh "$W/trial.sh" black-alder; zsh "$W/light.sh" black-alder; cat "$GEN/light/black-alder.txt"
cp "$GEN/try/black-alder.json" "$GEN/body/black-alder.json"; cp "$GEN/light/black-alder.txt" "$GEN/light/black-alder-body.txt"
zsh "$W/sheet.sh" "$1" "$GEN/body/black-alder.json" black-alder 50:gale:front 50:gale:plan
