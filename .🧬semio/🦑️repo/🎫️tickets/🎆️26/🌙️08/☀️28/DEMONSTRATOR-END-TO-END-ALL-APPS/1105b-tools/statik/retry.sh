#!/bin/bash
# usage: retry.sh <first-n> — reruns the statik pane smoke until the failure is not a transient pre-compiler source refusal
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; n=$1
while [ $n -lt $(( $1 + 40 )) ]; do
  "$C/1105b-tools/runtime/tst.sh" W-statik-$n "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/Cargo.toml" native -p semio-s-artifact-fem-3d-editor --test statik_pane_smoke -- statik_pane --nocapture --test-threads=1
  echo $n > "$C/🗑️generated/1105b/W-statik-last"
  /usr/bin/grep -aq "refused before compiler\|CargoLockRefused\|could not compile .semio-framework-\|could not compile .semio-os-\|could not compile .semio-plugin-" "$C/🗑️generated/1105b/W-statik-$n.txt" || break
  n=$((n+1)); sleep 240
done
touch "$C/🗑️generated/1105b/W-statik-retry.done"
