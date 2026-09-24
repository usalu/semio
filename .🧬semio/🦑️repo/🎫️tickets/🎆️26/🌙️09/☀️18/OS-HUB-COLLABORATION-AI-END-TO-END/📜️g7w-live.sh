#!/bin/zsh
# 🤝️ G7w: runs the two-user live law against hub 7900 into wp-g7w/generated/g7w-live-<tag>.txt (arg 1 = tag, arg 2 = runtime profile).
cd /Users/ueli/Documents/semio
export CARGO_TARGET_DIR="$PWD/.tmp-ticket-0918/wp-g7w/target" CARGO_INCREMENTAL=0 SEMIO_HUB_LIVE_ORIGIN=http://127.0.0.1:7900 SEMIO_HUB_LIVE_EMAIL=ada@example.org SEMIO_HUB_LIVE_PASSWORD='correct horse battery staple' SEMIO_HUB_LIVE_PEER_EMAIL=bo@example.org SEMIO_HUB_LIVE_PEER_PASSWORD='correct horse battery staple' SEMIO_PLUGIN=${SEMIO_PLUGIN:-block2d}
export SEMIO_PLUGIN_MODULES="$PWD/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/native/${2:-release}/$SEMIO_PLUGIN"
O=.tmp-ticket-0918/wp-g7w/generated/g7w-live-$1.raw.txt
cargo test -p semio-framework-os-renderer-wgpu --lib -- shell::hub_projection_workspace_tests::two_live_wgpu_shells_collaborate_on_one_hub_document --exact --ignored --nocapture > $O 2>&1
{ /usr/bin/grep -E '^error(\[|:)' -A6 $O | head -40; awk '/running 1 test/{f=1} f' $O | /usr/bin/grep -v 'render begin\|render leave'; } > ${O%.raw.txt}.txt
rm $O
/usr/bin/grep -E 'g7w-live step|test result|^error' ${O%.raw.txt}.txt | cut -c1-400
