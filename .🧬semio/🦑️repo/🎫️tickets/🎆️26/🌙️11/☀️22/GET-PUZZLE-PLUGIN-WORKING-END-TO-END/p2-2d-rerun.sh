#!/usr/bin/env bash
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PUZZLE-PLUGIN-WORKING-END-TO-END"
S="$T/🗑️generated/p2-2d-summary3.log"
until grep -q P2_ALL_DONE "$S"; do sleep 30; done
for pkg in $(grep '^P2_END' "$S" | grep -v 'status=0' | awk '{print $2}' | sort -u); do
  spec="$pkg"; [ "$pkg" = semio-s-artifact-puzzle-2d ] && spec="$pkg|--features component-app-assembly"
  for try in 1 2 3; do
    "$T/builder.sh" exec "cd '✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d' && P2_JOBS=3 bash $T/p2-2d-group2.sh 2d '$spec'"
    tail -c 300000 "$T/🗑️generated/p2-2d-$pkg.log" | grep -q 'signal: \(15\|9\)' || break
  done
done
echo "P2_RERUN_DONE"
