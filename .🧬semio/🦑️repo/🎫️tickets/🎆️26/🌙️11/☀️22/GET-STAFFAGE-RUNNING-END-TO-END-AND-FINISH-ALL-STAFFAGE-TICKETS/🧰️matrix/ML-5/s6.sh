# 🤸️ Stretching: what lies at the far places of v6, then v7 (hood, fringe, 1:500 head) with gate, and the same with the right sleeve swollen by 2 and 4 mm (guard only; sourced by q.sh).
echo "-- 1:100 front at 0.020,0.090"; cap 600 "$P" "$B" "$H/solids-at.ts" "$GEN/try/person-stretching.v6.json" --scale 100 --view front --at 0.020,0.090 --reach 0.004 | cut -c1-200
echo "-- 1:50 plan at -0.165,0.000"; cap 600 "$P" "$B" "$H/solids-at.ts" "$GEN/try/person-stretching.v6.json" --scale 50 --view plan --at -0.165,0.000 --reach 0.004 | cut -c1-200
source "$W/try.sh" person-stretching v7; cat "$GEN/try/person-stretching.v7.out" | cut -c1-300
for v in v8 v9; do source "$W/try.sh" person-stretching $v "loft prints guard"; cat "$GEN/try/person-stretching.$v.out" | cut -c1-300; done
