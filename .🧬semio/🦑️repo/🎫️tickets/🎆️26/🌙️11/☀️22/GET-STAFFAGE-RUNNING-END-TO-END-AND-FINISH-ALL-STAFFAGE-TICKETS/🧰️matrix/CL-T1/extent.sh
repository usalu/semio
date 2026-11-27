#!/bin/zsh
# 📐️ The extent of every body scene of the GLB of the default pose against `dimensions`, as the bounds check of `🧊️model-catalog` reads it (`../CL-3D/extent.ts`), for assets or trial copies inside the grant of its caller: extent.sh <folder under $GEN> <id | file.json>… writes `$GEN/<folder>/<id>-extent.txt`.
source "${0:A:h}/env.sh"
d="$GEN/$1"; mkdir -p "$d"; shift
for a in "$@"; do n="${${a:t}%.json}"; zsh "$W/run.sh" "$W3D/extent.ts" "$a" > "$d/$n-extent.txt" 2>&1; echo "$n exit $?"; grep -E "^s[0-9]+-body" "$d/$n-extent.txt" | cut -c1-130; done
