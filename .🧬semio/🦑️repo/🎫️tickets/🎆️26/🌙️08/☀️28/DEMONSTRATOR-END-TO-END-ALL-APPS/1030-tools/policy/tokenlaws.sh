#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
S=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$S/🗑️generated/1030"
while [ ! -f "$G/policy-taxonomy.rc" ]; do sleep 20; done
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path Cargo.toml --lib --no-fail-fast -p semio-s-artifact-procedural-2d-editor -p semio-s-artifact-procedural-3d-editor vcs_artifact_app > "$G/policy-tokenlaws.txt" 2>&1
echo $? > "$G/policy-tokenlaws.rc"
