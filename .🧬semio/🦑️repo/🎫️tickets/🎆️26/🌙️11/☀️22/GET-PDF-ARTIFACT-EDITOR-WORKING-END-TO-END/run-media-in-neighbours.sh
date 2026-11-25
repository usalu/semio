#!/bin/sh
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
for group in media_in shard_fuel_open natural_source_route natural_file_route registered_pdf_png_picker_publishes_an_atomic_revision_guarded_image_edit every_declared_pdf; do
  sh "$here/run-binary.sh" "mir-n-$group" "$group"
  echo "$group: $(grep -a 'test result' "$here/🗑️generated/mir-n-$group.log")"
done
touch "$here/🗑️generated/mir-n.done"
