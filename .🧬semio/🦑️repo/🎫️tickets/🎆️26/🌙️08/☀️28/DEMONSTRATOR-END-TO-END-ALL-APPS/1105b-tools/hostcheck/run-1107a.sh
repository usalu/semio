#!/bin/bash
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1105b"
while [ ! -f "$G/lock-sync-1107-ok" ]; do sleep 20; done
bash "$C/1105b-tools/hostcheck/tst.sh" hostcheck-101 "🧰️framework/Cargo.toml" -p semio-framework-artifact-reference -p semio-os-store-link-schema
if [ "$(cat $G/hostcheck-101.rc)" = 0 ]; then python3 "$C/1105b-tools/hostcheck/apply-artifact.py" go child; fi
bash "$C/1105b-tools/chk.sh" hostcheck-102 "🧰️framework/Cargo.toml" native --tests -p semio-os-store-child
echo done > "$G/hostcheck-1107a.done"
