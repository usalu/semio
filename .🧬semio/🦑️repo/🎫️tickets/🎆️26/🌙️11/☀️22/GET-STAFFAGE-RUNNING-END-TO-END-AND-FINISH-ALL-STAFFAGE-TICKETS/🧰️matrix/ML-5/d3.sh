# 🐕️ Guide dog: the palm of the side elevation as a loft, three variants (guard and prints first, then the gate; sourced by q.sh).
for v in h2 h3 h4; do SCALES=50 source "$W/try.sh" person-with-guide-dog $v "loft guard gate scan"; cat "$GEN/try/person-with-guide-dog.$v.out" | cut -c1-300; done
