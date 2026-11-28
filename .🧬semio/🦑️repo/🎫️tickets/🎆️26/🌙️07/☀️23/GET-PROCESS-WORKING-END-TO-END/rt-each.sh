#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; N=$T/🗑️generated/native; G=$T/🗑️generated
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$N/target CARGO_BUILD_BUILD_DIR=$N/build
export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off
cd "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d" || exit 1
for s in "" -host -examples -io -viewer -inferences -reference-solids -editor -model -commands -mut-stock -mut-machines -mut-steps -mutations -model-diff -presence -kit -config -panels -wasm -modes; do
  c=semio-s-artifact-process-process3d$s
  for try in 1 2 3; do
    cargo test -j 2 --no-fail-fast -p $c > $G/rust-tests-u-$c.txt 2>&1; rc=$?
    [ $rc -eq 0 ] && break
    grep -q 'SIGTERM' $G/rust-tests-u-$c.txt || break
  done
  echo "$c exit $rc try $try" >> $G/rust-tests-u-summary.txt
done
echo done >> $G/rust-tests-u-summary.txt
