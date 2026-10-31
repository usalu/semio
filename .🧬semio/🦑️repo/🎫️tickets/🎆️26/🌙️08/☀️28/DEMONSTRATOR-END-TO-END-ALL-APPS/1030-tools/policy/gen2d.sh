#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
S=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$S/🗑️generated/1030"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/Cargo.toml" --lib --no-fail-fast -p semio-s-artifact-procedural-2d-host -p semio-s-artifact-procedural-2d-editor vcs_artifact_app > "$G/policy-app-gen2d.txt" 2>&1
echo $? > "$G/policy-app-gen2d.rc"
