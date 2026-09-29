#!/bin/zsh
# ✅️ T14 (14c, coordinator item 2): the hub create path of generation 2d/3d, natively — S19's genesis laws
# (`a_hub_genesis_pair_is_produced_and_parses_back_without_trapping`, the SDK `artifact_app_genesis_pair` = hub `codec.genesis`)
# and archive-door laws on the LIVE tree after T1. Native lane, build-fleet-b + private target.
export CARGO_INCREMENTAL=0 NX_DAEMON=false RUST_MIN_STACK=33554432
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-t14/target
cd /Users/ueli/Documents/semio || exit 2
echo "LANE $(date +%T)"
for crate in semio-s-artifact-procedural-generation2d semio-s-artifact-procedural-generation3d; do
  cargo test --no-fail-fast -p $crate --features component-app-assembly --lib -- a_hub_genesis_pair archive 2>&1 | /usr/bin/grep -E ': error|^error|^test |test result|panicked' ; echo "RC $crate ${pipestatus[1]} $(date +%T)"
done
