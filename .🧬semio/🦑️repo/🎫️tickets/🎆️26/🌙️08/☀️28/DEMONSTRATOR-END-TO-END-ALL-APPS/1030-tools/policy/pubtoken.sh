#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
S=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$S/🗑️generated/1030"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path Cargo.toml --lib --keep-going -p semio-framework-plugin -p semio-framework-os-store-one-item-batch -p semio-s-artifact-procedural-2d-host -p semio-s-artifact-procedural-2d-editor -p semio-s-artifact-procedural-3d-host -p semio-s-artifact-procedural-3d-editor -p semio-s-artifact-process-process3d-host -p semio-s-artifact-process-process3d-editor -p semio-s-artifact-raster-raster-host -p semio-s-artifact-raster-raster-editor -p semio-s-artifact-demonstrator-playground-editor -p semio-s-artifact-playbook-playbook-v1 > "$G/policy-pubtoken.txt" 2>&1
echo $? > "$G/policy-pubtoken.rc"
