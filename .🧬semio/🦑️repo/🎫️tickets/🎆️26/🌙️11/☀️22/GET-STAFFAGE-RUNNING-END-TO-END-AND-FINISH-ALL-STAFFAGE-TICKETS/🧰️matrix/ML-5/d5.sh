# 🐕️ Guide dog: the print refusals of the trial in full, then gate and scan of h5 (sourced by q.sh).
cap 600 "$T" "$B" "$U/🧰️matrix/MC/prints.ts" "$GEN/try/person-with-guide-dog.h5.json" 2>&1 | cut -c1-1500
source "$W/try.sh" person-with-guide-dog h5 "gate scan"; cat "$GEN/try/person-with-guide-dog.h5.out" | cut -c1-300
