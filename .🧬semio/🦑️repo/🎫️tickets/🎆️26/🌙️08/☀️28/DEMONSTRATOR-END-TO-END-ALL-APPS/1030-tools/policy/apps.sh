#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
S=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$S/🗑️generated/1030"
A="✏️s/🔌️plugins"
run() { name="$1"; shift; SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- "$@" > "$G/policy-app-$name.txt" 2>&1; echo $? > "$G/policy-app-$name.rc"; }
run gen2d test --manifest-path "$A/🌀️procedural/🗿️artifacts/🌀️generation2d/Cargo.toml" --lib --no-fail-fast -p semio-s-artifact-procedural-2d-host -p semio-s-artifact-procedural-2d-editor vcs_artifact_app
run gen3d test --manifest-path "$A/🌀️procedural/🗿️artifacts/🧊️generation3d/Cargo.toml" --lib --no-fail-fast -p semio-s-artifact-procedural-3d-host -p semio-s-artifact-procedural-3d-editor vcs_artifact_app
run p3d check --manifest-path "$A/🏭️process/🗿️artifacts/🧊️process3d/Cargo.toml" --lib --keep-going -p semio-s-artifact-process-process3d-host -p semio-s-artifact-process-process3d-editor
run raster check --manifest-path "$A/🖨️raster/🗿️artifacts/🖨️raster/Cargo.toml" --lib --keep-going -p semio-s-artifact-raster-raster-host -p semio-s-artifact-raster-raster-editor
run playground check --manifest-path "$A/🎪️demonstrator/🗿️artifacts/🎪️playground/Cargo.toml" --lib --keep-going -p semio-s-artifact-demonstrator-playground-editor
run playbook check --manifest-path "$A/📖️playbook/🗿️artifacts/📖️playbook/Cargo.toml" --lib --keep-going -p semio-s-artifact-playbook-playbook-v1
