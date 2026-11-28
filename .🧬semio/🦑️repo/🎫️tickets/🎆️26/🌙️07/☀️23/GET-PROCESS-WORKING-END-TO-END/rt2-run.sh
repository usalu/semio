#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; N=$T/🗑️generated/native-tests; G=$N
export PATH="/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/:$PATH"
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$N/target CARGO_BUILD_BUILD_DIR=$N/build
export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off
R=/Users/ueli/Documents/semio
run() { dir=$1; name=$2; shift 2
  cd "$dir" || return
  for try in 1 2 3 4; do
    cargo test -j 2 --no-fail-fast "$@" > $G/$name.txt 2>&1; rc=$?
    [ $rc -eq 0 ] && break
    grep -q 'SIGTERM' $G/$name.txt || break
  done
  echo "$name exit $rc try $try" >> $G/summary.txt
}
run "$R/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d" editor -p semio-s-artifact-process-process3d-editor
run "$R/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d" examples -p semio-s-artifact-process-process3d-examples
run "$R/🌎️hub/🧩️compositions/🏭️process" hub -p semio-hub-process -p semio-hub-process-viewer-app -p semio-hub-process-editor-app
echo done >> $G/summary.txt
