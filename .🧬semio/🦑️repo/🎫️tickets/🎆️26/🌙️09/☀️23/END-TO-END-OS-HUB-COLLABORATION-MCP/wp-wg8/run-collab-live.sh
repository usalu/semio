#!/bin/zsh
# WG8: G7w's two-user wgpu law (hub-live-collaboration-check's law) against W2's hub 7800 on the full catalog,
# with the block2d release runtime staged from the one tree. Credentials from wp-w2's Hub Handoff.
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0
export CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-wg8/target
if ! bun "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📦️modules/📜️script.ts" publish block2d release; then
  # WG8_CATALOG_COMPONENT: the hub catalog's block component; the staged runtime is kept only when its component bytes are exactly those.
  [ -n "$WG8_CATALOG_COMPONENT" ] || exit 3
  staged=$(shasum -a 256 "✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/dist/component-release/semio_s_plugin_block.wasm" | cut -d' ' -f1)
  catalog=$(shasum -a 256 "$WG8_CATALOG_COMPONENT" | cut -d' ' -f1)
  [ "$staged" = "$catalog" ] || exit 4
  echo "WG8 keeps the staged runtime: component $staged = hub catalog component; release descriptor not yet rematerialized"
fi
export SEMIO_PLUGIN=block2d
export SEMIO_PLUGIN_MODULES="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/native/release/block2d"
export SEMIO_HUB_LIVE_ORIGIN=${SEMIO_HUB_LIVE_ORIGIN:-http://127.0.0.1:7800}
export SEMIO_HUB_LIVE_EMAIL=user1@semio.dev
export SEMIO_HUB_LIVE_PASSWORD=gm1-local-dev-pass-1
export SEMIO_HUB_LIVE_PEER_EMAIL=user2@semio.dev
export SEMIO_HUB_LIVE_PEER_PASSWORD=gm1-local-dev-pass-2
cargo test -p semio-framework-os-renderer-wgpu --lib --no-fail-fast -- shell::hub_projection_workspace_tests::two_live_wgpu_shells_collaborate_on_one_hub_document --exact --ignored --nocapture
echo "EXIT=$?"
