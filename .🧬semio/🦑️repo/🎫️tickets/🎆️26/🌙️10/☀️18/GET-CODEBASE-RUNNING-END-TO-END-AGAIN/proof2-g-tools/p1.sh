#!/bin/sh
# proof 1: the four process-birth laws and the birth-free digest laws, sequentially, each its own official `repo exec`
cd "$(dirname "$0")/../../../../../../../.." || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; G="$C/🗑️generated/proof2-g/p1"
run() { s=$(date +%s); sh "$C/proof2-g-tools/env-digest.sh" "$1" "$2" > "$G/$3.log" 2>&1; rc=$?; echo "$3 rc=$rc wall=$(( $(date +%s) - s ))s" >> "$G/rows.txt"; }
run snapshot "a batch hashes" t1
run snapshot "an entry edited" t2
run digest "exact Source actual batch streams" t3
run digest "replaced mid-read" t4
run digest "agrees with schema|extent budgets|require managed" t5
echo finished >> "$G/rows.txt"
