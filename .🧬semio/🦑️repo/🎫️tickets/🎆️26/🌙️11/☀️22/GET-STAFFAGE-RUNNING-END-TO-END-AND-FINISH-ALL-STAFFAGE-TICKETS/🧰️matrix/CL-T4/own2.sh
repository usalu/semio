#!/bin/zsh
# 🧮️ `own2.ts` over assets or trial copies inside the grant of its caller: own2.sh <folder under $GEN> <id | file.json> [own2 options…] writes `$GEN/<folder>/<id>[-<pose>].txt`.
source "${0:A:h}/env.sh"
d="$GEN/$1"; mkdir -p "$d"; a="$2"; shift 2; n="${${a:t}%.json}"; p=""; [[ "$*" == *--pose* ]] && p="-${@[$(( ${@[(i)--pose]} + 1 ))]}"
zsh "$W/run.sh" own2.ts "$a" "$@" > "$d/$n$p.txt" 2>&1; echo "$n$p exit $? $(head -1 "$d/$n$p.txt" | cut -c1-200)"
