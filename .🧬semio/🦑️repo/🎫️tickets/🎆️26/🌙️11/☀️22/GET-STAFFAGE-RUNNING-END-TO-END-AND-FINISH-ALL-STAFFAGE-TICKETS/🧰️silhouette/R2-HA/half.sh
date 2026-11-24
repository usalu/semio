#!/bin/zsh
# 🌗️ Redraws limbs of one view as ONE half profile each (a body of revolution about its bone without mirror, the exact outline of its solids in a side or front view) in the work ops, from the state before this round (the backup): the ops of the shape and of its lengths `<id>-2` … are taken back first. half.sh <slug> <scale> <elevation|front> <id[=solid+solid…]>…
source "${0:A:h}/env.sh"
a="$1"; sc="$2"; v="$3"; shift 3; o="$GEN/ops/$a.json"; bk="$GEN/backup/$a.json"
ids=(); specs=()
for s in "$@"; do i="${s%%=*}"; ids+=("$i" "$i-2" "$i-3" "$i-4" "$i-5" "$i-6"); specs+=("$i=${${s#*=}:-$i}@solid=half"); done
cd "$S/📦️packages/🟦️typescript"
timeout 60 "$B" "$W/dropop.ts" "$o" "$sc" "$v" $ids
timeout 150 "$B" "$W/outline.ts" "$bk" "$sc" "$v" "$o" $specs --base "$o" | grep -v "off 0.0[0-2]" | cut -c1-160
