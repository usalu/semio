#!/bin/zsh
cd /Users/ueli/Documents/semio
G=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END/🗑️generated
run() { n=$1; shift; repo cargo --store=shared --configuration=prototype --parallelism=2 -- test --no-fail-fast "$@" > $G/rust-tests-g-$n.txt 2>&1; echo "exit $?" > $G/rust-tests-g-$n.done; }
run bridge -p semio-process-mutation-bridge
run oracle -p semio-s-plugin-process-catalog-json-oracle
run metal -p semio-s-plugin-process-metal -p semio-s-plugin-process-metal-component
run wood -p semio-s-plugin-process-wood -p semio-s-plugin-process-wood-component
run robotic -p semio-s-plugin-process-robotic -p semio-s-plugin-process-robotic-component
run concrete -p semio-s-plugin-process-concrete -p semio-s-plugin-process-concrete-component
run process3d  -p semio-s-artifact-process-process3d -p semio-s-artifact-process-process3d-host -p semio-s-artifact-process-process3d-examples -p semio-s-artifact-process-process3d-io -p semio-s-artifact-process-process3d-viewer -p semio-s-artifact-process-process3d-inferences -p semio-s-artifact-process-process3d-reference-solids -p semio-s-artifact-process-process3d-editor -p semio-s-artifact-process-process3d-model -p semio-s-artifact-process-process3d-commands -p semio-s-artifact-process-process3d-mut-stock -p semio-s-artifact-process-process3d-mut-machines -p semio-s-artifact-process-process3d-mut-steps -p semio-s-artifact-process-process3d-mutations -p semio-s-artifact-process-process3d-model-diff -p semio-s-artifact-process-process3d-presence -p semio-s-artifact-process-process3d-kit -p semio-s-artifact-process-process3d-config -p semio-s-artifact-process-process3d-panels -p semio-s-artifact-process-process3d-wasm -p semio-s-artifact-process-process3d-harness -p semio-s-artifact-process-process3d-modes
run hub -p semio-hub-process -p semio-hub-process-viewer-app -p semio-hub-process-editor-app
echo all > $G/rust-tests-g-all.done
