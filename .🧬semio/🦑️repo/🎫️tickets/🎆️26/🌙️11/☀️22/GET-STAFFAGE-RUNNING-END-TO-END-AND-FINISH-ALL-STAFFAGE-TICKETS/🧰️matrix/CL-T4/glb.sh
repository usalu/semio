#!/bin/zsh
# 📦️ `glb.ts` over assets or trial copies, one after the other inside the grant of its caller: glb.sh <folder under $GEN for the outputs> <id | file.json>… writes `$GEN/<folder>/<id>.txt` and prints the size line of each.
source "${0:A:h}/env.sh"
d="$GEN/$1"; mkdir -p "$d"; shift
for a in "$@"; do n="${${a:t}%.json}"; zsh "$W/run.sh" glb.ts "$a" > "$d/$n.txt" 2>&1; echo "$n exit $? $(grep -m1 '^glb ' "$d/$n.txt" | cut -c1-200)"; done
