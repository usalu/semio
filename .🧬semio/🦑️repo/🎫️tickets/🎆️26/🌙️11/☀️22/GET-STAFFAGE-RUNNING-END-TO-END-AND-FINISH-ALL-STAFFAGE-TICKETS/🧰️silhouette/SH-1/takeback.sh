#!/bin/zsh
# ↩️ Takes back, at the given scales of a person, exactly the repaired shapes (with their group) that make `validateAsset` or the print suitability of the check CLI report something the file of the session start did not; no new repair. takeback.sh <slug> <scale>…
source "${0:A:h}/env.sh"
a="$1"; shift
for s in "$@"; do print -n "" | "$W/hand.sh" "$a" "$s" | sed "s/^/$a 1:$s /"; done
