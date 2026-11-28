#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; N=$T/🗑️generated/native; G=$T/🗑️generated
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$N/target CARGO_BUILD_BUILD_DIR=$N/build
export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off
R=/Users/ueli/Documents/semio
run() { n=$1; d=$2; shift 2; (cd "$R/$d" && cargo test -j 2 --no-fail-fast "$@" > $G/rust-tests-p-$n.txt 2>&1; echo "exit $?" > $G/rust-tests-p-$n.done); }
run bridge "✏️s/🔌️plugins/🏭️process/🏭️bridge" 
run oracle "✏️s/🔌️plugins/🏭️process/🧩️extensions/🧪️tests/🔬️catalog-json" 
run metal "✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal" -p semio-s-plugin-process-metal -p semio-s-plugin-process-metal-component
run wood "✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood" -p semio-s-plugin-process-wood -p semio-s-plugin-process-wood-component
run robotic "✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic" -p semio-s-plugin-process-robotic -p semio-s-plugin-process-robotic-component
run concrete "✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete" -p semio-s-plugin-process-concrete -p semio-s-plugin-process-concrete-component
run process3d "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d"  -p semio-s-artifact-process-process3d -p semio-s-artifact-process-process3d-host -p semio-s-artifact-process-process3d-examples -p semio-s-artifact-process-process3d-io -p semio-s-artifact-process-process3d-viewer -p semio-s-artifact-process-process3d-inferences -p semio-s-artifact-process-process3d-reference-solids -p semio-s-artifact-process-process3d-editor -p semio-s-artifact-process-process3d-model -p semio-s-artifact-process-process3d-commands -p semio-s-artifact-process-process3d-mut-stock -p semio-s-artifact-process-process3d-mut-machines -p semio-s-artifact-process-process3d-mut-steps -p semio-s-artifact-process-process3d-mutations -p semio-s-artifact-process-process3d-model-diff -p semio-s-artifact-process-process3d-presence -p semio-s-artifact-process-process3d-kit -p semio-s-artifact-process-process3d-config -p semio-s-artifact-process-process3d-panels -p semio-s-artifact-process-process3d-wasm -p semio-s-artifact-process-process3d-harness -p semio-s-artifact-process-process3d-modes
run hub "🌎️hub" -p semio-hub-process -p semio-hub-process-viewer-app -p semio-hub-process-editor-app
echo all > $G/rust-tests-p-all.done
