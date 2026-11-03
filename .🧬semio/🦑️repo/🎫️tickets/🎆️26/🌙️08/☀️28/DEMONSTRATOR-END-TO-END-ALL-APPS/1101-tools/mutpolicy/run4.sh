#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G=$C/🗑️generated/1101
A="✏️s/🔌️plugins"
while [ ! -f $G/mutpolicy-r4-done ]; do sleep 20; done
rm -f $G/mutpolicy-r5-*.rc $G/mutpolicy-r5-done
r(){ n=$1; shift; SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- "$@" > $G/mutpolicy-r5-$n.txt 2>&1; echo $? > $G/mutpolicy-r5-$n.rc; }
r home test --manifest-path "$A/🪐️space/🗿️artifacts/🏠️home/Cargo.toml" --lib -p semio-s-artifact-space-home-editor -- the_fold_stepper
r epw test --manifest-path "$A/🗄️stdio/🗿️artifacts/🌦️epw/Cargo.toml" --lib -p semio-s-artifact-stdio-epw-mutations -- semio_payload_law
r gltf test --manifest-path "$A/🗄️stdio/🗿️artifacts/🧊️gltf/Cargo.toml" --lib -p semio-s-artifact-stdio-gltf-mutations -- semio_payload_law
r energy test --manifest-path "$A/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml" --lib -p semio-s-artifact-energy-model-mutations -- semio_payload_law
r tail test --manifest-path 🧰️framework/Cargo.toml --lib -p semio-framework-replication-mutation roster_uniqueness
echo done > $G/mutpolicy-r5-done
