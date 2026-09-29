#!/bin/zsh
# ✅️ T14 flow record-spec root fix gate (live tree): native `--lib --tests` check of flow + its 9 extensions + the describe crate,
# then the flow codec's pack-schema law, then (wasm lane, separately) `bun nx run @semio-tech/flow-plugin:describe`.
export CARGO_INCREMENTAL=0 NX_DAEMON=false RUST_MIN_STACK=33554432
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-t14/target
cd /Users/ueli/Documents/semio || exit 2
echo "LANE $(date +%T)"
P=(-p semio-s-artifact-flow-flow -p semio-s-plugin-flow -p semio-s-plugin-flow-extension-list -p semio-s-plugin-flow-extension-draw -p semio-s-plugin-flow-extension-text -p semio-s-plugin-flow-extension-logic -p semio-s-plugin-flow-extension-primitive -p semio-s-plugin-flow-extension-math -p semio-s-plugin-flow-extension-brep -p semio-s-plugin-flow-extension-dictionary -p semio-s-plugin-flow-extension-bim -p semio-framework-plugin-describe)
cargo check --keep-going ${P[@]} --lib --tests --message-format short 2>&1 | /usr/bin/grep -E ': error|^error|generated [0-9]+ warning|Finished' ; echo "CHECK-RC ${pipestatus[1]} $(date +%T)"
cargo test --no-fail-fast -p semio-s-artifact-flow-flow --lib -- artifact_kind pack record_spec codec 2>&1 | /usr/bin/grep -E ': error|^error|test result|FAILED|panicked' ; echo "TEST-RC ${pipestatus[1]} $(date +%T)"
