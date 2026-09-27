#!/bin/zsh
# WG10 s13: the kernel laws of WG10's landing (canonical pair decoder/admission/client/actor seed, transport deadline, codec registry).
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-wg10/target CARGO_BUILD_BUILD_DIR=${WG10_BUILD_DIR:-/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b}
echo "START $(date '+%F %T')"
nice -n 10 cargo test -p semio-framework-os-kernel --features sync,ureq --lib --no-fail-fast -- canonical_checkpoint_pair canonical_pair a_seeded_hub_actor request_budget slow_answer kind_resolves_to_its_linked mounted_component_codec_is_the_kind native_terminal_connection_failure
echo "RC=$? END $(date '+%F %T')"
