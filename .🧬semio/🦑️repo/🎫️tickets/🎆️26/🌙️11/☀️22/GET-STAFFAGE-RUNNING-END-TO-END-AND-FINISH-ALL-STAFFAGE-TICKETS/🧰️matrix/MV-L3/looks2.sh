#!/bin/zsh
# 🖼️ The panels of both trees of round MV-L3 inside one grant: `looks2.sh` draws the body copies (`looks.sh`) of `field-maple` in `storm` and of `black-alder` in `gale`, then the loft copy of `black-alder` in `gale` at 1:50 (front, plan) and 1:200 (front).
source "${0:A:h}/env.sh"
zsh "$W/looks.sh" field-maple storm summer-calm
zsh "$W/looks.sh" black-alder gale summer-calm
zsh "$W/sheet.sh" loft "$GEN/loft/black-alder.json" black-alder 50:gale:front 50:gale:plan 200:gale:front
