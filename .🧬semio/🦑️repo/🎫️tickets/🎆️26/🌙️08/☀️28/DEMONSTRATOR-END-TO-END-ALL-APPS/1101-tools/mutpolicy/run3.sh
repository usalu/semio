#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G=$C/🗑️generated/1101
A="✏️s/🔌️plugins"
rm -f $G/mutpolicy-r4-*.rc $G/mutpolicy-r4-done
r(){ n=$1; shift; SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- "$@" > $G/mutpolicy-r4-$n.txt 2>&1; echo $? > $G/mutpolicy-r4-$n.rc; }
r p3d check --manifest-path "$A/🏭️process/🗿️artifacts/🧊️process3d/Cargo.toml" --lib --keep-going -p semio-s-artifact-process-process3d-host -p semio-s-artifact-process-process3d-editor
r raster check --manifest-path "$A/🖨️raster/🗿️artifacts/🖨️raster/Cargo.toml" --lib --keep-going -p semio-s-artifact-raster-raster-host -p semio-s-artifact-raster-raster-editor
r gen3d test --manifest-path "$A/🌀️procedural/🗿️artifacts/🧊️generation3d/Cargo.toml" --lib --no-fail-fast -p semio-s-artifact-procedural-3d-host -p semio-s-artifact-procedural-3d-editor vcs_artifact_app
echo done > $G/mutpolicy-r4-done
