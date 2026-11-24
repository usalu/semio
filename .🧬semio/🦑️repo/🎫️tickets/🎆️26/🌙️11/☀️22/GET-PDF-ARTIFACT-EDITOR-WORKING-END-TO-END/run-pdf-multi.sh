#!/bin/sh
# usage: run-pdf-multi.sh <name>  (snapshot-value filters, then the content unit crate; each cargo goes through the global gate)
export PATH=$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH RUST_MIN_STACK=67108864
export WATCHDOG_TEST_SECONDS=${WATCHDOG_TEST_SECONDS:-600}
cd "$(dirname "$0")"
G=🗑️generated
name=$1
python3 watchdog-cargo.py sh ./native-cargo.sh test -p semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests --lib -- controlled_pdf_terminal_publication_requires_both_owned_headers registered_pdf_page_and_authoring_actions_undo_and_redo_step_by_step registered_pdf_png_picker_publishes_an_atomic_revision_guarded_image_edit registered_pdf_font_change_preserves_encoded_text_and_the_next_line registered_pdf_with_several_fonts_images_and_inherited_resources --test-threads=1 --nocapture > $G/$name-snapshot.log 2>&1
python3 watchdog-cargo.py sh ./native-cargo.sh test -p semio-s-artifact-stdio-pdf-mut-family-8 --lib -- --test-threads=1 > $G/$name-content.log 2>&1
echo done > $G/$name.done
