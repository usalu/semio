# 🐕️ Guide dog: where the bands 100 and 200 do not print at 1:150 (sourced by q.sh).
for band in 100 200; do cap 900 "$T" "$B" "$W/prints-at.ts" "$GEN/try/person-with-guide-dog.h5.json" $band 150 2>&1 | cut -c1-300; done
