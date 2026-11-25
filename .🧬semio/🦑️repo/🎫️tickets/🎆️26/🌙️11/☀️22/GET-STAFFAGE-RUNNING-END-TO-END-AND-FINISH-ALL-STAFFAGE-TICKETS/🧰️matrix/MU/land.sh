#!/bin/zsh
# 🛬️ Lands the repair of one asset: writes it (apply.sh), the cheap proofs (scan, validate, print, check), the silhouette and sweep measure of the untouched and the repaired document, and a contact sheet with the untouched document in the first row: land.sh <id> [<scale>|<query>]…   (run it inside a grant)
source "${0:h}/env.sh"
a="$1"; shift; mkdir -p "$GEN/look"
"$W/apply.sh" "$a"
"$W/steps.sh" scan,validate,print,check "$a"
rows=("$GEN/backup/$a.json|$1"); for s in "$@"; do rows+=("$a|$s"); done
(( $# > 0 )) && (cd "$W" && "$B" sheet.ts --panel ${PANEL:-560x400} "$GEN/look/$a.png" "${rows[@]}")
cmp -s "$(asset "$a")" "$GEN/backup/$a.json" && echo "UNCHANGED"
echo landed
