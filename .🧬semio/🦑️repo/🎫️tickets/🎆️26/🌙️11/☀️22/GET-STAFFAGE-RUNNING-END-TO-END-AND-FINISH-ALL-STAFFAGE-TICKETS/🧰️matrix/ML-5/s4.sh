# 🔬️ Stretching: which loft of v6 moves the guard (one left out per variant; sourced by q.sh).
for s in 50 100; do for part in sleeve-l sleeve-r forearm-r forearm-l hair hoodie hem; do
  SCALES=$s source "$W/try.sh" person-stretching "x$s-$part" "guard"; echo "without $part: $(grep "1:$s " "$GEN/try/person-stretching.x$s-$part.out" | cut -c1-200)"
done; done
