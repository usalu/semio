#!/bin/bash
# usage: gate.sh <n> — hub gate then 21 ext components (labels value-<n>*, ext round <n>); touches hub-ok-1107 / ext-ok-1107 only on rc 0
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; D="$C/1105b-tools"; G="$C/🗑️generated/1105b"; N=$1
rm -f "$G/value-$N-gate.rc"
"$D/hub.sh" "value-$N-hub"
H=$(cat "$G/value-$N-hub.rc"); [ "$H" = 0 ] && touch "$G/hub-ok-1107"
"$D/value/ext.sh" "$N"
E=$(cat "$G/value-$N.rc"); [ "$E" = 0 ] && touch "$G/ext-ok-1107"
echo "hub=$H ext=$E" > "$G/value-$N-gate.rc"
