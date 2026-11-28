#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; N=$T/🗑️generated/native-tests
export PATH="/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/:$PATH"
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$N/target CARGO_BUILD_BUILD_DIR=$N/build
export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off
R=/Users/ueli/Documents/semio; P="$R/✏️s/🔌️plugins/🏭️process"
run() { dir=$1; name=$2; shift 2; cd "$dir" || return
  cargo test -j 2 --no-fail-fast "$@" > $N/$name.txt 2>&1; echo "$name exit $?" >> $N/summary2.txt; }
for e in 🔩️metal 🪵️wood 🤖️robotic 🧱️concrete; do run "$P/🧩️extensions/$e" ext-$(echo $e | tr -d '\0' | od -An -tx1 | tr -d ' \n' | cut -c1-8) --workspace; done
run "$P/🏭️bridge" bridge --workspace
run "$P/🗿️artifacts/🧊️process3d" p3d-examples -p semio-s-artifact-process-process3d-examples
run "$P/🗿️artifacts/🧊️process3d" p3d-model -p semio-s-artifact-process-process3d-model
run "$R/🌎️hub/🧩️compositions/🏭️process" hub -p semio-hub-process -p semio-hub-process-viewer-app -p semio-hub-process-editor-app
echo done >> $N/summary2.txt
