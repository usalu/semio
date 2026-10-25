#!/bin/sh
# usage: chain.sh name=[--tool=x ]project:target ...  (sequential; summary in 🗑️generated/pres-d/chain-summary.txt)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
for a in "$@"; do
  n=${a%%=*}; t=${a#*=}
  case "$t" in default:*) sh "$C/pres-d-tools/run.sh" "$n" run "${t#default:}";; *) sh "$C/pres-d-tools/run.sh" "$n" --tool=bazel run "$t";; esac
  echo "$n $t $(tail -n 1 "$C/🗑️generated/pres-d/$n.raw") $(date +%H:%M)" >> "$C/🗑️generated/pres-d/chain-summary.txt"
done
