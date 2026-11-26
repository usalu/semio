#!/bin/zsh
# 🧪️ The guard of the TRIAL pocket module of MX-2 (`../MX-2/pocket/silhouette.ts`, not the live one) on copies, inside one grant, to know before the landing whether a copy will pass (both modules are loaded: 1795 MB measured, reserve 2300 MB): `pocket-trial.sh <file.json>…` prints the lines of `../MX-2/pocket/measure.ts`.
source "${0:A:h}/env.sh"
cd "$S/📦️packages/🟦️typescript"
for f in "$@"; do timeout 2400 "$B" "$U/🧰️matrix/MX-2/pocket/measure.ts" "$f" 2>&1 | cut -c1-260; done
