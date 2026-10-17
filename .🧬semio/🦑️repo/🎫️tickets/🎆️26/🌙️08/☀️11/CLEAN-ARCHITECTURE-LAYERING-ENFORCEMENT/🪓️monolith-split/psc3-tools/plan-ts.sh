#!/bin/sh
# Plans every open owner that has a TypeScript package. Usage: plan-ts.sh <fresh-label> <plan-label> [extra plan args...]
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G="$F/🗑️generated"
cd "$F" || exit 2
L="$1"; P="$2"; shift 2
bun psc3-tools/groups.ts "$G/psc3-policy-$L.json" "$G/psc3-scan-$L.json" > "$G/psc3-groups-$L.txt"
set --
while IFS= read -r o; do set -- "$@" --owner "$o"; done <<LIST
$(node -e '
const fs=require("fs"); const lines=fs.readFileSync(process.argv[1],"utf8").split("\n").filter(Boolean);
for (const l of lines){const c=l.split("\t"); const m=/ts=(\S+)/.exec(l); if(m&&m[1]!=="NONE"){ console.log(c[1].replace(/F\/M\//g,"🧰️framework/🔨️modules/").replace(/F\/P\//g,"🧰️framework/🛍️products/")) }}' "$G/psc3-groups-$L.txt")
LIST
bun psc3-tools/plan.ts "$G/psc3-policy-$L.json" "$G/psc3-scan-$L.json" "$G/psc3-plan-$P.json" "$@" --exclude-target "🧰️framework/🔨️modules/🖱️ui/"
