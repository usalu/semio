#!/bin/zsh
# 🌿️ `groups.ts` over several assets, one after the other inside the grant of its caller: groups.sh <folder under $GEN for the outputs> <id | file.json>… writes `$GEN/<folder>/<id>.txt` (the `TOP` heaviest groups per scene, default 14).
source "${0:A:h}/env.sh"
d="$GEN/$1"; mkdir -p "$d"; shift
for a in "$@"; do n="${${a:t}%.json}"; zsh "$W/run.sh" groups.ts "$a" --top ${TOP:-14} > "$d/$n.txt" 2>&1; echo "$n exit $? $(tail -1 "$d/$n.txt" | cut -c1-200)"; done
