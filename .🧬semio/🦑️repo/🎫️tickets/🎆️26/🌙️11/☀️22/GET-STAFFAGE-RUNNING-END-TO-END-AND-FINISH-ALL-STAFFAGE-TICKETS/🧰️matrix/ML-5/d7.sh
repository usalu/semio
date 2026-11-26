# 🐕️ Guide dog: the whole trial h7 (hands of the side elevation, undrawn hand solids, pens of band 200), then where band 200 stands at 1:150 (sourced by q.sh).
source "$W/try.sh" person-with-guide-dog h7; cat "$GEN/try/person-with-guide-dog.h7.out" | cut -c1-300
cap 900 "$T" "$B" "$W/prints-at.ts" "$GEN/try/person-with-guide-dog.h7.json" 200 150 2>&1 | cut -c1-300
source "$W/sheet.sh" x before=base/person-with-guide-dog.json:50 after=try/person-with-guide-dog.h7.json:50 after=try/person-with-guide-dog.h7.json:50:walking-with-guide-dog after=try/person-with-guide-dog.h7.json:200
