#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1105b"
M="🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📦️packages/🦀️rust/Cargo.toml"
while :; do
  t=$(bun $C/1105b-tools/capsplit/closure.ts "$M" testkit "" test 2>/dev/null | tail -1 | awk '{print $2}')
  [ -n "$t" ] && [ "$t" -le 262144 ] && break
  sleep 120
done
n=140
while :; do
  "$C/1105b-tools/nativetests/tst.sh" nativetests-$n 🧰️framework/Cargo.toml -p semio-framework-value-native-owned -p semio-framework-value-persistent -p semio-framework-value-snapshot-read -p semio-framework-value-tests -p semio-framework-value-tests-2 --lib --tests
  /usr/bin/grep -aq "Source refused before compiler: semio-framework-job-context\|CargoLockRefused:" "$G/nativetests-$n.txt" || break
  n=$((n+1)); sleep 240
done
echo $n > "$G/nativetests-final2.n"
