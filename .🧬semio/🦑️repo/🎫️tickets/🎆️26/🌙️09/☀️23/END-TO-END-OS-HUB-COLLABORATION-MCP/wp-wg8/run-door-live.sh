#!/bin/zsh
# WG8: the live creation-door law against W2's canonical hub 7800 (credentials from wp-w2's Hub Handoff).
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0
export CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-wg8/target
export SEMIO_HUB_LIVE_ORIGIN=http://127.0.0.1:7800
export SEMIO_HUB_LIVE_EMAIL=user1@semio.dev
export SEMIO_HUB_LIVE_PASSWORD=gm1-local-dev-pass-1
cargo test -p semio-framework-os-renderer-wgpu --lib --no-fail-fast -- shell::hub_projection_workspace_tests::a_live_hub_artifact_is_created_through_the_wgpu_creation_door --exact --ignored --nocapture
echo "EXIT=$?"
