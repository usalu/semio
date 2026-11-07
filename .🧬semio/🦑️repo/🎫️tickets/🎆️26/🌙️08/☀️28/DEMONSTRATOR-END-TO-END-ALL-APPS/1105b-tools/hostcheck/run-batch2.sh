#!/bin/bash
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
while [ ! -f "$C/🗑️generated/1105b/W-hostcheck-68.rc" ]; do sleep 15; done
G="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf"
bash $C/1105b-tools/chk.sh W-hostcheck-69 "$G/Cargo.toml" native --tests -p semio-s-artifact-stdio-gltf-editor
python3 $C/1105b-tools/hostcheck/apply-artifact.py
bash $C/1105b-tools/chk.sh W-hostcheck-70 "🧰️framework/Cargo.toml" native --tests -p semio-framework-artifact-reference
