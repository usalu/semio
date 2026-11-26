#!/bin/zsh
# Builds the Process hub component for the browser prototype with plain cargo (the `repo run` graph phase does not finish here). The wasm profile and the published artifact path (publish-hub.ts reads $N/target) are outside `repo cargo`, so the build is admitted into the machine-wide ledger through `repo capacity run`; its jobs follow the grant.
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; N=$T/🗑️generated/native
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$N/target CARGO_BUILD_BUILD_DIR=$N/build
export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off
mkdir -p $N
cd "/Users/ueli/Documents/semio/🌎️hub/🧩️compositions/🏭️process/🧩️component/📦️packages/🦀️rust" || exit 1
for i in $(seq 1 ${HUB_ATTEMPTS:-8}); do
  repo capacity run --memory=7680 --cpu=3 -- "$T/hub-cargo.sh" build --target wasm32-wasip2 --profile wasm-prototype --config 'profile.wasm-prototype.inherits="dev"' --config 'profile.wasm-prototype.opt-level="s"' --config profile.wasm-prototype.codegen-units=16 --config 'profile.wasm-prototype.lto="thin"' --config profile.wasm-prototype.incremental=false > $T/🗑️generated/hub-build.txt 2>&1 && break
  grep -q "^error\[E" $T/🗑️generated/hub-build.txt || break
  sleep 120
done
echo "exit $?" > $T/🗑️generated/hub-build.done
