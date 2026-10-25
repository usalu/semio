#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
L="sh $C/verify-d-tools/lane.sh"
echo "--- chain5 start $(date +%H:%M)" >> "$C/🗑️generated/verify-d/verdicts.tsv"
AMB="🧰️framework/📦️packages/🟦️typescript/🌿️ambient"
$L tool-spawns.3 tool-spawns
$L tool-declarations.3 tool-declarations
$L build-owners.3 build-owners
$L package-purity.3 package-purity
$L ts-process.2 typescript check --scope 🧰️framework/🔨️modules/🏃️process --scope "$AMB" --timeout-minutes 25
$L ts-dashboard.4 typescript check --scope 🧰️framework/🛍️products/🦑️repo --scope "$AMB" --timeout-minutes 25
$L ts-course.2 typescript check --scope 🎓️teaching --scope "$AMB" --timeout-minutes 25
$L ts-print.4 typescript check --scope 🧰️framework/🛍️products/📓️print --scope 🧰️framework/🛍️products/🎤️presentation --scope ♻️mit-bestand/🎤️präsentation --scope "$AMB" --timeout-minutes 25
echo CHAIN5-DONE >> "$C/🗑️generated/verify-d/verdicts.tsv"
