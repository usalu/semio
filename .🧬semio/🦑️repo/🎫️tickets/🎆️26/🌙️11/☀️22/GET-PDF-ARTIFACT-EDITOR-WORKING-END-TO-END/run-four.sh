#!/bin/sh
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
for t in controlled_pdf_terminal_publication_requires_both_owned_headers registered_pdf_font_change_preserves_encoded_text_and_the_next_line registered_pdf_page_and_authoring_actions_undo_and_redo_step_by_step registered_pdf_png_picker_publishes_an_atomic_revision_guarded_image_edit; do
  RUST_BACKTRACE=1 perl -e 'alarm 600; exec @ARGV' sh "$here/run-binary.sh" "four-$t" "$t" --exact "mirror::editor::pdf17::tests::$t"
done
echo done > "$here/🗑️generated/four.done"
