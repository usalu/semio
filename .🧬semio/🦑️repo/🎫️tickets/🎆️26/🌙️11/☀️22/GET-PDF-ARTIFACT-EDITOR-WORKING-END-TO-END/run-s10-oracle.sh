#!/bin/sh
# usage: run-s10-oracle.sh  (slice 10: the pymupdf oracle over the three fixtures, then over the files the registered Save wrote from them (SEMIO_PDF_SLICE_TEN_DIR))
cd "$(dirname "$0")"
F="../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧪️tests/🧫️fixtures"
uv run --offline --with pymupdf==1.26.7 python -I form-images-oracle.py "$F"
for kind in form-images page-resources acro-form; do
  [ -f "🗑️generated/real-world/$kind.saved.pdf" ] && uv run --offline --with pymupdf==1.26.7 python -I form-images-oracle.py "$F" "$kind" "🗑️generated/real-world/$kind.saved.pdf"
done
