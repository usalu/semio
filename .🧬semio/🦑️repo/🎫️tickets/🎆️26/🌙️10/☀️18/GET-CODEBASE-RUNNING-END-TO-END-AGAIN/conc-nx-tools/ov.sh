#!/bin/sh
# rebuilds the overlay from the live tree, applies conc-cargo group lease when the live tree lacks it, then the staged conc-nx set
cd "$(dirname "$0")" && python3 ops_gen.py >/dev/null && python3 mirror.py /private/tmp/conc-nx/ov --real-library >/dev/null
if ! grep -q withCompilerSharedHeadroomV1 "/private/tmp/conc-nx/ov/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts"; then
  # conc-cargo mirrors only its own targets: copy the capacity module (real) before it patches the overlay
  (cd ../conc-cargo-tools && CONC_CARGO_ROOT=/private/tmp/conc-nx/ov CONC_CARGO_BACKUP=/private/tmp/conc-nx/work/cargo-backup python3 apply.py apply --write --groups lease --ignore-sync >/dev/null 2>&1) || echo "conc-cargo lease group could not be applied to the overlay"
fi
python3 apply.py apply --write --root /private/tmp/conc-nx/ov --ignore-running | tail -1
