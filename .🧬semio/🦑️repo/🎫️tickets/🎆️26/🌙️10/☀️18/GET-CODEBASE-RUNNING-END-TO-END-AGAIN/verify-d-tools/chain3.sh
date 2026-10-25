#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
L="sh $C/verify-d-tools/lane.sh"
echo "--- chain3 start $(date +%H:%M)" >> "$C/🗑️generated/verify-d/verdicts.tsv"
$L agnostic-framework-names agnostic framework-names
$L agnostic-utility-machines agnostic utility-machines
$L rust-warnings-scope rust-warnings --scope-only
$L ts-course typescript check --scope 🎓️teaching --timeout-minutes 25
$L ts-print typescript check --scope 🧰️framework/🛍️products/📓️print --scope 🧰️framework/🛍️products/🎤️presentation --scope ♻️mit-bestand/🎤️präsentation --timeout-minutes 25
$L ts-dashboard typescript check --scope 🧰️framework/🛍️products/🦑️repo --timeout-minutes 25
$L ts-process typescript check --scope 🧰️framework/🔨️modules/🏃️process --timeout-minutes 25
$L layering layering
echo CHAIN3-DONE >> "$C/🗑️generated/verify-d/verdicts.tsv"
