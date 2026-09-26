#!/bin/zsh
# 🧪️ G10: wgpu AgentBridge + AgentPresence twin unit laws (native lib tests) in G10's private target.
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust" || exit 1
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-g10/target
date; nice -n 15 cargo test -p semio-framework-os-renderer-wgpu --lib --no-fail-fast -- agent_bridge agent_presence agent_chat_panel wgpu_agent_overlays; echo "RC=$?"; date
