#!/bin/zsh
# 🪞️ Compares the evaluated boxes of every shape of an asset in a pose with and without the ops (differences only; the solid marks are dropped): restdiff.sh <id> <pose> <scale>…
source "${0:A:h}/env.sh"
a="$1"; pose="$2"; shift 2
for s in "$@"; do
  diff <(pb probe.ts "$a" "$s" "$pose" --no-ops | sed -E 's/ (extruded|revolved):[uvw]//; s/ followTip//') <(pb probe.ts "$a" "$s" "$pose" | sed -E 's/ (extruded|revolved):[uvw]//; s/ followTip//') | grep '^[<>]' | head -${N:-30}
done
