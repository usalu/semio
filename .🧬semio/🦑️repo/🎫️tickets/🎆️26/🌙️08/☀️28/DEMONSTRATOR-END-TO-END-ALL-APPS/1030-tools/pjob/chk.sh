#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1
rm -f "$C/🗑️generated/1030/pjob-$N.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path 🧰️framework/Cargo.toml --target wasm32-unknown-unknown --keep-going --lib -p semio-framework-job-worker-authority -p semio-framework-job-worker -p semio-framework-ui-prepared -p semio-framework-ui-wgpu-mounted-layout -p semio-framework-ui-runtime-presentation > "$C/🗑️generated/1030/pjob-$N.txt" 2>&1
echo $? > "$C/🗑️generated/1030/pjob-$N.rc"
