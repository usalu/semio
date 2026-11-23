#!/bin/zsh
# 🚢️ Writes the catalog file of people from the generator output and every stage of ops, then runs the gates (gate.sh). ship.sh <slug>…
source "${0:A:h}/env.sh"
for a in "$@"; do python3 "$W/apply.py" $a "$GEN/base/$a.json" "$CAT/$a/🔣️.json" > /dev/null && "$W/gate.sh" $a; done
