#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; N=$T/🗑️generated/native; G=$T/🗑️generated
export PATH="/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/:$PATH"
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$N/target CARGO_BUILD_BUILD_DIR=$N/build
export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off
cd "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d" || exit 1

for i in 1 2 3 4; do cargo test -j 2 --no-fail-fast -p semio-s-artifact-process-process3d-editor > $G/rust-tests-x-editor.txt 2>&1; rc=$?; [ $rc -eq 0 ] && break; grep -q SIGTERM $G/rust-tests-x-editor.txt || break; cp $G/rust-tests-x-editor.txt $G/rust-tests-x-editor-try$i.txt; done; echo "exit $rc try $i" > $G/rust-tests-x-editor.done
