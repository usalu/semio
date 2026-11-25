#!/bin/sh
# Round 2 of the picker law: the framework abort-port law, the picker law and its editor neighbours, then the native-save unit target, one gated cargo at a time.
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
cd "$here" || exit 1
G=🗑️generated
export RUST_MIN_STACK=67108864 WATCHDOG_TEST_SECONDS=900
store="$here/../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🛑️original-batch-abort/📦️packages/🦀️rust"
NATIVE_CARGO_DIR="$store" python3 watchdog-cargo.py sh ./native-cargo.sh test --lib -- --test-threads=1 --nocapture > "$G/r2-store-abort.log" 2>&1
python3 watchdog-cargo.py sh ./native-cargo.sh test -p semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests --lib -- \
  registered_pdf_png_picker_publishes_an_atomic_revision_guarded_image_edit \
  registered_pdf_page_edit_publishes_and_undoes_redoes \
  registered_pdf_draft_publishes_once_refuses_stale_and_undoes_redoes \
  registered_pdf_with_several_fonts_images_and_inherited_resources_opens_like_both_reference_readers \
  natural_file_route_edits_title_and_inserts_text_then_saves_both_pages \
  media_in_of_a_small_pdf_then_edit_and_save_appends_to_the_opened_file \
  a_progressing_media_in_is_never_faulted_as_stalled \
  --test-threads=1 > "$G/r2-editor-neighbours.log" 2>&1
python3 watchdog-cargo.py sh ./native-cargo.sh test -p semio-s-artifact-stdio-pdf-native-save --lib -- --test-threads=1 > "$G/r2-native-save.log" 2>&1
echo done > "$G/r2-suite.done"
