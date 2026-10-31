#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
S=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$S/🗑️generated/1029"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path 🧰️framework/Cargo.toml --lib --keep-going -p semio-framework-tool-machine -p semio-framework-tool-typing-edit -p semio-framework-pack-record-retained -p semio-framework-plugin-instance-render -p semio-framework-plugin-instance-typed-ops -p semio-framework-plugin-original-job-ownership > "$G/policysweep-fw.txt" 2>&1
echo $? > "$G/policysweep-fw.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path Cargo.toml --lib --keep-going -p semio-s-artifact-energy-model-editor-inspector -p semio-s-artifact-energy-model-editor-commands -p semio-s-artifact-energy-model-mutations > "$G/policysweep-energy.txt" 2>&1
echo $? > "$G/policysweep-energy.rc"
