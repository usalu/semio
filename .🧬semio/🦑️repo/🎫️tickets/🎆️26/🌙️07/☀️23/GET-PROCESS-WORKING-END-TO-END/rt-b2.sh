#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; B=$T/🗑️generated/rt-build; G=$T/🗑️generated
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$B/target CARGO_BUILD_BUILD_DIR=$B/build
export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off
export PATH=$PATH:$HOME/.bun/bin
R=/Users/ueli/Documents/semio
cd "$R/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d" && cargo test -j 2 --no-fail-fast -p semio-s-artifact-process-process3d-editor > $G/rt-b2-editor.txt 2>&1; echo "exit $?" > $G/rt-b2-editor.done
cd "$R/🌎️hub/🧩️compositions/🏭️process" && cargo test -j 2 --no-fail-fast -p semio-hub-process -p semio-hub-process-viewer-app -p semio-hub-process-editor-app > $G/rt-b2-hub.txt 2>&1; echo "exit $?" > $G/rt-b2-hub.done
