# 🐕️ Guide dog: what lies at the two far places of the hand trial, then the undrawn solids of the hands as lofts (three views, side elevation only; sourced by q.sh).
for at in 0.215,0.880 0.239,0.818; do echo "-- at $at"; cap 600 "$P" "$B" "$H/solids-at.ts" "$GEN/try/person-with-guide-dog.h1.json" --scale 50 --view elevation --at $at --reach 0.004 | cut -c1-200; done
for v in h5 h6; do SCALES=50 source "$W/try.sh" person-with-guide-dog $v "loft prints guard"; cat "$GEN/try/person-with-guide-dog.$v.out" | cut -c1-300; done
