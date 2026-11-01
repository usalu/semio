#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G=$C/🗑️generated/1101; T=$C/1101-tools
A="✏️s/🔌️plugins"
rm -f $G/mutpolicy-r2-*.rc $G/mutpolicy-r2-done
"$T/chk.sh" mutpolicy-r2-fem2d "$A/🏗️fem/🗿️artifacts/◻️2d/Cargo.toml" native --lib -p semio-s-artifact-fem-2d-editor-transient
r(){ n=$1; shift; SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- "$@" > $G/mutpolicy-r2-$n.txt 2>&1; echo $? > $G/mutpolicy-r2-$n.rc; }
r law test --manifest-path Cargo.toml -p semio-framework-os-store-one-item-batch --test mutation_apply cold_replay
r gen2d test --manifest-path "$A/🌀️procedural/🗿️artifacts/🌀️generation2d/Cargo.toml" --lib --no-fail-fast -p semio-s-artifact-procedural-2d-host -p semio-s-artifact-procedural-2d-editor vcs_artifact_app
r gen3d test --manifest-path "$A/🌀️procedural/🗿️artifacts/🧊️generation3d/Cargo.toml" --lib --no-fail-fast -p semio-s-artifact-procedural-3d-host -p semio-s-artifact-procedural-3d-editor vcs_artifact_app
r p3d check --manifest-path "$A/🏭️process/🗿️artifacts/🧊️process3d/Cargo.toml" --lib --keep-going -p semio-s-artifact-process-process3d-host -p semio-s-artifact-process-process3d-editor
r raster check --manifest-path "$A/🖨️raster/🗿️artifacts/🖨️raster/Cargo.toml" --lib --keep-going -p semio-s-artifact-raster-raster-host -p semio-s-artifact-raster-raster-editor
r derive check --manifest-path 🧰️framework/Cargo.toml --lib --keep-going -p semio-framework-replication-mutation-retained
echo done > $G/mutpolicy-r2-done
