#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1105b"
n=${1:-110}
while :; do
  "$C/1105b-tools/nativetests/tst.sh" nativetests-$n 🧰️framework/Cargo.toml -p semio-framework-value-native-owned -p semio-framework-value-persistent -p semio-framework-value-snapshot-read -p semio-framework-value-tests -p semio-framework-value-tests-2 --lib --tests
  /usr/bin/grep -aq "Source refused before compiler: semio-framework-job-context\|CargoLockRefused:" "$G/nativetests-$n.txt" || break
  n=$((n+1)); sleep 240
done
echo done > "$G/nativetests-final.marker"
echo $n > "$G/nativetests-final.n"
