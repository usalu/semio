#!/bin/zsh
# 🖼️ The last panels of round MV-L3 inside one grant: the chairs at 1:500 before (backup) and after (catalog) in the stored pose and in use, and the written alder (the trial copy, equal to the catalog file) at 1:50 in `gale` and `early-wet-snow`.
source "${0:A:h}/env.sh"
c=used-bentwood-dining-chairs
HEIGHT=900 zsh "$W/sheet.sh" before "$GEN/backup/$c.json" $c 500:stored:front 500:in-use:front 500:in-use:plan
HEIGHT=900 zsh "$W/sheet.sh" final "$(ls "$C"/*/"$c"/🔣️.json)" $c 500:stored:front 500:stored:plan 500:stored:elevation 500:in-use:front 500:in-use:plan 500:in-use:elevation
zsh "$W/sheet.sh" final "$GEN/try/black-alder.json" black-alder 50:gale:front 50:early-wet-snow:front 50:gale:plan 100:gale:plan 200:gale:front
