#!/bin/bash
# g6: the native genesis solve's own process CPU (user+sys) and wall, from the already built test binary; $1 = capture name.
BIN=$(ls -t /Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build/debug/build/semio-s-artifact-wfc-bitmap/*/out/semio_s_artifact_wfc_bitmap-* 2>/dev/null | /usr/bin/grep -v '\.d$' | head -1)
cd "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/📦️packages/🦀️rust"
/usr/bin/time -p "$BIN" the_artifact_bound_genesis_document_resolves_and_solves --exact standards::v1::subsets::any::schema::inferences::component::tests::the_artifact_bound_genesis_document_resolves_and_solves > "/Users/ueli/Documents/semio/.tmp-ticket/wp-g6/generated/$1.txt" 2>&1
