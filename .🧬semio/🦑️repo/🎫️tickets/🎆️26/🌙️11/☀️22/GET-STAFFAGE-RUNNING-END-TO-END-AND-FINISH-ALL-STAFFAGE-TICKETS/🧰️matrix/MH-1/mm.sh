#!/bin/zsh
# 🤝️ Runs the next job of lane MM (`U/🧰️matrix/MM/next.sh`, at most three, each content once) inside the grant of the lane that calls it, before the lane's own job; a script without `timeout` or with a git command is refused: mm.sh   (counted in $GEN/mm.log by md5)
f="$U/🧰️matrix/MM/next.sh"; l="$GEN/mm.log"; touch "$l"
[[ -f "$f" ]] || exit 0
sum="$(md5 -q "$f")"; grep -q "$sum" "$l" && exit 0
(( $(grep -c started "$l") >= 3 )) && exit 0
grep -q timeout "$f" && ! grep -qw git "$f" || { echo "$sum refused" >> "$l"; exit 0 }
echo "$sum started $(date +%H:%M:%S)" >> "$l"; cp "$f" "$GEN/mm-$sum.sh"; (unset INSIDE W GEN BATCH; cd "${f:h}" && zsh "$f" > /dev/null 2>&1); echo "$sum exit $? $(date +%H:%M:%S)" >> "$l"
