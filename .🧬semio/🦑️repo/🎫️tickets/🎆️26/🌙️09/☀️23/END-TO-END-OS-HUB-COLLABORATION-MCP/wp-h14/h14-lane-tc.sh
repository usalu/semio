#!/bin/zsh
# ⚖️ H14 14b: native lane — the three red hub trusted_catalog laws of hold 3 with their full panic output; capture → <capture>.
OUT=$1
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h14/target
echo "=== queued $(date +%T)" > "$OUT"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native h14 -- zsh -c '
echo "=== start $(date +%T)"
nice -n 15 cargo test -p semio-hub --lib --no-fail-fast -- guest_codec_tables_answer_like_the_linked_native_codecs gis_map_binding_constructs_from_loaded_catalog_and_refuses_tampered_retained_bytes linked_stdio_gis_descriptor_failures_never_publish_a_partial_codec_closure --test-threads 1 2>&1 | /usr/bin/grep -v "^warning\|^ *|\|^ *= \|^ *-->" | tail -120; echo "=== TC EXIT ${pipestatus[1]} $(date +%T)"
' >> "$OUT" 2>&1
echo "=== EXIT $? $(date +%T)" >> "$OUT"
