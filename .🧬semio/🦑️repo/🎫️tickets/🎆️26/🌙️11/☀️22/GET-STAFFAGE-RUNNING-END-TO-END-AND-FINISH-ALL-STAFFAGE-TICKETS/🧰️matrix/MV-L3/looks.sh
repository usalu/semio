#!/bin/zsh
# 🖼️ The panels of round MV-L3 for one tree inside one grant: `looks.sh <id> <wind pose> <calm pose>` draws the trial copy in the wind pose at 1:50, 1:100 and 1:200 in front and plan (1:50 also the elevation), the backup (plates) at 1:50 in front and plan of the wind pose, and the default pose of the copy at 1:50 in front and plan.
source "${0:A:h}/env.sh"
a="$1"; w="$2"; c="$3"
zsh "$W/sheet.sh" try "$GEN/try/$a.json" "$a" 50:${w}:front 50:${w}:plan 50:${w}:elevation 100:${w}:front 100:${w}:plan 200:${w}:front 200:${w}:plan 50:${c}:front 50:${c}:plan
zsh "$W/sheet.sh" before "$GEN/backup/$a.json" "$a" 50:${w}:front 50:${w}:plan
